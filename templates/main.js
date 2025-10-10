const badge_rows = () => Array.from(document.querySelectorAll("table#badges > tbody > tr"));
const get_checkbox = tr => tr.querySelector('input[type="checkbox"');

const select_none = () => {
    badge_rows().forEach(tr => get_checkbox(tr).checked = false);
    update_selected();
}

const select_tickets = (paid_only = true) => {
    badge_rows().filter(tr => {
        let c = tr.children[3].className;
        return c == "Paid" || (!paid_only && (c == "Unpaid" || c == "Unknown"));
    }).forEach(tr => get_checkbox(tr).checked = true);
    update_selected();
}

const select_sponsors = (paid_only = true) => {
    badge_rows().filter(tr => {
        let c = tr.children[4].className;
        return c == "Paid" || (!paid_only && (c == "Unpaid" || c == "Unknown"));
    }).forEach(tr => get_checkbox(tr).checked = true);
    update_selected();
}

const deselect_noavatar = () => {
    badge_rows().filter(tr => {
        let c = tr.children[6].className;
        return c == "No";
    }).forEach(tr => get_checkbox(tr).checked = false);
    update_selected();
}

const filter_users = (value) => {
    badge_rows().forEach(tr => {
        const search = value.toLowerCase();
        const badge_num = tr.children[1].textContent.toLowerCase();
        const name = tr.children[2].textContent.toLowerCase();
        const match = badge_num.indexOf(search) != -1 || name.indexOf(search) != -1;
        if (match) {
            tr.style.display = "";
        } else {
            tr.style.display = "none";
        }
    });
}

const update_selected = () => {
    console.log("updte_selected", badge_rows()[0]);
    badge_rows().forEach(tr => {
        if (get_checkbox(tr).checked) {
            tr.style["font-weight"] = "bold";
        } else {
            tr.style["font-weight"] = "";
        }
    })
}