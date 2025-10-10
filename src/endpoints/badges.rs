use image::EncodableLayout;
use printpdf::{
    FontId, Op, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, Point, Pt, Px, RawImage,
    TextAlign, TextShapingOptions, XObjectId, XObjectTransform,
};
use sqlx::{MySql, Pool};
use std::fs;

use crate::{
    AppError,
    templates::UserEntry,
    types::{Layer, LayersConfig},
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
    font: FontId,
}

impl BadgePDF {
    pub async fn init(db: &Pool<MySql>, name: &str) -> Result<Self, AppError> {
        let layers_config =
            sqlx::query_as::<_, LayersConfig>("select * from layers_config limit 1;")
                .fetch_one(db)
                .await
                .map_err(AppError::Db)?;
        let layers = sqlx::query_as::<_, Layer>("select * from layers;")
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
                    let img = fs::read(&layer.asset_path).unwrap();
                    let img = RawImage::decode_from_bytes(img.as_bytes(), &mut Vec::new()).unwrap();
                    let id = Some(doc.add_image(&img));
                    LayerId { layer, id }
                }
            })
            .collect::<Vec<_>>();

        let font = fs::read(&layers_config.font_path).unwrap();
        let font = ParsedFont::from_bytes(font.as_bytes(), 0, &mut vec![]).unwrap();
        let font = doc.add_font(&font);

        let pages = Vec::new();
        Ok(Self {
            doc,
            config: layers_config,
            layers,
            pages,
            font,
        })
    }

    pub fn add_user(&mut self, user: &UserEntry) -> Result<(), AppError> {
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
                ops.push(Op::UseXobject {
                    id: layer.id.clone().unwrap(),
                    transform: Default::default(),
                })
            }
        }

        let options = TextShapingOptions {
            font_size: self.config.regnum_size(),
            ..Default::default()
        };
        let shaped_text = self
            .doc
            .shape_text(&format!("#{}", user.regnumber), &self.font, &options)
            .unwrap();
        let text_drawing_ops = shaped_text.get_ops(Point {
            x: self.config.regnum_x(),
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
            .shape_text(&user.nickname, &self.font, &options)
            .unwrap();
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
                ops.push(Op::UseXobject {
                    id: layer.id.clone().unwrap(),
                    transform: Default::default(),
                })
            }
        }

        let options = TextShapingOptions {
            font_size: self.config.regnum_size(),
            ..Default::default()
        };
        let shaped_text = self
            .doc
            .shape_text(
                &format!(
                    "{}",
                    user.fursuit_species
                        .as_ref()
                        .unwrap_or(&"unknown".to_owned())
                ),
                &self.font,
                &options,
            )
            .unwrap();
        let text_drawing_ops = shaped_text.get_ops(Point {
            x: self.config.regnum_x(),
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
            .shape_text(
                user.fursuit_name.as_ref().unwrap_or(&"unknown".to_owned()),
                &self.font,
                &options,
            )
            .unwrap();
        let text_drawing_ops = shaped_text.get_ops(Point {
            x: Pt(0.0),
            y: self.config.nick_y(),
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
