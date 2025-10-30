use image::EncodableLayout;
use printpdf::{
    Color, FontId, Op, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, Point, Pt, Px, RawImage,
    Rgb, TextAlign, TextShapingOptions, XObjectId, XObjectTransform,
};
use sqlx::{MySql, Pool};
use std::fs;

use crate::{
    AppError,
    templates::UserEntry,
    types::{BadgeOpt, Layer, LayerOpt, LayersConfig},
};

struct LayerId {
    layer: Layer,
    id: Option<XObjectId>,
}

pub struct BadgePDF {
    doc: PdfDocument,
    config: LayersConfig,
    layers: Vec<LayerId>,
    pages: Vec<PdfPage>,
    name_font: FontId,
    nr_font: FontId,
}

impl BadgePDF {
    pub async fn init(db: &Pool<MySql>, name: &str) -> Result<Self, AppError> {
        let uploads_dir = std::env::var("UPLOADS_DIR").unwrap();
        let layers_config =
            sqlx::query_as::<_, LayersConfig>("select * from layers_config limit 1;")
                .fetch_one(db)
                .await
                .map_err(AppError::Db)?;
        let layers = sqlx::query_as::<_, Layer>("select * from layers order by id;")
            .fetch_all(db)
            .await
            .map_err(AppError::Db)?;

        let mut doc = PdfDocument::new(name);
        let layers = layers
            .into_iter()
            .map(|layer| {
                if layer.asset_path == "badge" {
                    LayerId { layer, id: None }
                } else {
                    let img = fs::read(format!("{}/{}", uploads_dir, layer.asset_path)).unwrap();
                    let img = RawImage::decode_from_bytes(img.as_bytes(), &mut Vec::new()).unwrap();
                    let id = Some(doc.add_image(&img));
                    LayerId { layer, id }
                }
            })
            .collect::<Vec<_>>();

        let name_font =
            fs::read(format!("{}/{}", uploads_dir, layers_config.name_font_path)).unwrap();
        let name_font = ParsedFont::from_bytes(name_font.as_bytes(), 0, &mut vec![]).unwrap();
        let name_font = doc.add_font(&name_font);

        let nr_font = fs::read(format!("{}/{}", uploads_dir, layers_config.nr_font_path)).unwrap();
        let nr_font = ParsedFont::from_bytes(nr_font.as_bytes(), 0, &mut vec![]).unwrap();
        let nr_font = doc.add_font(&nr_font);

        let pages = Vec::new();
        Ok(Self {
            doc,
            config: layers_config,
            layers,
            pages,
            name_font,
            nr_font,
        })
    }

    pub fn add_user(&mut self, user: &UserEntry, add_media: bool) -> Result<(), AppError> {
        let mut ops = Vec::new();
        for layer in self.layers.iter() {
            if layer.layer.asset_path == "badge" {
                let img = fs::read(&user.avatar).unwrap();
                let img = RawImage::decode_from_bytes(img.as_bytes(), &mut Vec::new()).unwrap();
                let id = self.doc.add_image(&img);
                ops.push(Op::UseXobject {
                    id,
                    transform: XObjectTransform {
                        translate_x: Some(Pt((self.config.page_width().into_pt()
                            - self.config.avatar_size())
                        .0 / 2.0)),
                        translate_y: Some(self.config.avatar_y()),
                        rotate: None,
                        scale_x: Some(
                            self.config.avatar_size().0 / Px(img.width).into_pt(self.config.dpi).0,
                        ),
                        scale_y: Some(
                            self.config.avatar_size().0 / Px(img.height).into_pt(self.config.dpi).0,
                        ),
                        dpi: Some(300.0),
                    },
                });
            } else {
                let should_not = layer.layer.event_not;
                let mut should_print = layer.layer.badge_type.is_any();
                if layer.layer.badge_type.is_con_ticket() && !user.ticket_convention.is_no() {
                    should_print = true;
                }
                if layer.layer.badge_type.is_day_ticket() && !user.ticket_day.is_no() {
                    should_print = true;
                }
                if add_media && !layer.layer.event.is_media() {
                    should_print = false;
                }
                if should_print {
                    should_print = match layer.layer.event {
                        LayerOpt::Any => !should_not,
                        LayerOpt::Fursuit => false,
                        LayerOpt::Staff => user.staff.is_no() == should_not,
                        LayerOpt::Media => add_media,
                        LayerOpt::Sponsor => {
                            user.staff.is_no() && user.sponsor.is_no() == should_not
                        }
                        LayerOpt::Ticket => {
                            user.staff.is_no() && user.ticket_any.is_no() == should_not
                        }
                        LayerOpt::TicketConvention => {
                            user.staff.is_no() && user.ticket_convention.is_no() == should_not
                        }
                        LayerOpt::TicketWed => {
                            user.staff.is_no() && user.ticket_wed.is_no() == should_not
                        }
                        LayerOpt::TicketThu => {
                            user.staff.is_no() && user.ticket_thu.is_no() == should_not
                        }
                        LayerOpt::TicketFri => {
                            user.staff.is_no() && user.ticket_fri.is_no() == should_not
                        }
                        LayerOpt::TicketSat => {
                            user.staff.is_no() && user.ticket_sat.is_no() == should_not
                        }
                        LayerOpt::TicketSun => {
                            user.staff.is_no() && user.ticket_sun.is_no() == should_not
                        }
                    };
                }
                if should_print {
                    ops.push(Op::UseXobject {
                        id: layer.id.clone().unwrap(),
                        transform: Default::default(),
                    })
                }
            }
        }

        let options = TextShapingOptions {
            font_size: self.config.regnum_size(),
            // max_width: Some(Pt(50.0)),
            max_width: Some(self.config.page_width().into_pt()),
            // align: TextAlign::Right,
            align: TextAlign::Center,
            ..Default::default()
        };
        let shaped_text = self
            .doc
            .shape_text(&format!("{}", user.regnumber), &self.nr_font, &options)
            .unwrap();
        let c = self
            .config
            .regnum_color
            .parse::<csscolorparser::Color>()
            .unwrap();
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: c.r,
                g: c.g,
                b: c.b,
                icc_profile: None,
            }),
        });
        let text_drawing_ops = shaped_text.get_ops(Point {
            // x: self.config.regnum_x(),
            x: Pt(0.0),
            y: self.config.regnum_y(),
        });
        ops.extend_from_slice(&text_drawing_ops);

        let options = TextShapingOptions {
            font_size: self.config.nick_size(),
            max_width: Some(self.config.page_width().into_pt()),
            align: TextAlign::Center,
            ..Default::default()
        };
        let shaped_text = self
            .doc
            .shape_text(&user.nickname, &self.name_font, &options)
            .unwrap();
        let c = self
            .config
            .nick_color
            .parse::<csscolorparser::Color>()
            .unwrap();
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: c.r,
                g: c.g,
                b: c.b,
                icc_profile: None,
            }),
        });
        let text_drawing_ops = shaped_text.get_ops(Point {
            x: Pt(0.0),
            y: self.config.nick_y(),
        });
        ops.extend_from_slice(&text_drawing_ops);

        let page = PdfPage::new(self.config.page_width(), self.config.page_height(), ops);
        self.pages.push(page);

        Ok(())
    }

    pub fn add_fursuit(&mut self, user: &UserEntry) -> Result<(), AppError> {
        let mut ops = Vec::new();
        for layer in self.layers.iter() {
            if layer.layer.asset_path == "badge" {
                let img = fs::read(&user.fursuit_avatar).unwrap();
                let img = RawImage::decode_from_bytes(img.as_bytes(), &mut Vec::new()).unwrap();
                let id = self.doc.add_image(&img);
                ops.push(Op::UseXobject {
                    id,
                    transform: XObjectTransform {
                        translate_x: Some(Pt((self.config.page_width().into_pt()
                            - self.config.avatar_size())
                        .0 / 2.0)),
                        translate_y: Some(self.config.avatar_y()),
                        rotate: None,
                        scale_x: Some(
                            self.config.avatar_size().0 / Px(img.width).into_pt(self.config.dpi).0,
                        ),
                        scale_y: Some(
                            self.config.avatar_size().0 / Px(img.height).into_pt(self.config.dpi).0,
                        ),
                        dpi: Some(300.0),
                    },
                });
            } else {
                let should_not = layer.layer.event_not;
                let mut should_print = match layer.layer.badge_type {
                    BadgeOpt::Any | BadgeOpt::Fursuit => true,
                    _ => false,
                };
                if should_print {
                    should_print = match layer.layer.event {
                        LayerOpt::Any => !should_not,
                        LayerOpt::Fursuit => true,
                        _ => false,
                    };
                }
                if should_print {
                    ops.push(Op::UseXobject {
                        id: layer.id.clone().unwrap(),
                        transform: Default::default(),
                    })
                }
            }
        }

        let fursuit_species = user.fursuit_species.as_deref().unwrap_or("thing");
        let options = TextShapingOptions {
            font_size: self.config.regnum_size() - Pt(2.0),
            // max_width: Some(Pt(50.0)),
            max_width: Some(self.config.page_width().into_pt()),
            // align: TextAlign::Right,
            align: TextAlign::Center,
            ..Default::default()
        };
        let shaped_text = self
            .doc
            .shape_text(fursuit_species, &self.nr_font, &options)
            .unwrap();
        let c = self
            .config
            .regnum_color
            .parse::<csscolorparser::Color>()
            .unwrap();
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: c.r,
                g: c.g,
                b: c.b,
                icc_profile: None,
            }),
        });
        let text_drawing_ops = shaped_text.get_ops(Point {
            // x: self.config.regnum_x(),
            x: Pt(0.0),
            y: self.config.regnum_y() - Pt(7.0),
        });
        ops.extend_from_slice(&text_drawing_ops);

        let options = TextShapingOptions {
            font_size: self.config.nick_size(),
            max_width: Some(self.config.page_width().into_pt()),
            align: TextAlign::Center,
            ..Default::default()
        };
        let fursuit_name = user.fursuit_name.as_deref().unwrap_or("some");

        let shaped_text = self
            .doc
            .shape_text(fursuit_name, &self.name_font, &options)
            .unwrap();
        let c = self
            .config
            .nick_color
            .parse::<csscolorparser::Color>()
            .unwrap();
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: c.r,
                g: c.g,
                b: c.b,
                icc_profile: None,
            }),
        });
        let text_drawing_ops = shaped_text.get_ops(Point {
            x: Pt(0.0),
            y: self.config.nick_y() + Pt(12.0),
        });
        ops.extend_from_slice(&text_drawing_ops);

        let page = PdfPage::new(self.config.page_width(), self.config.page_height(), ops);
        self.pages.push(page);

        Ok(())
    }

    pub fn print(mut self) -> Vec<u8> {
        self.doc
            .with_pages(self.pages)
            .save(&PdfSaveOptions::default(), &mut Vec::new())
    }
}
