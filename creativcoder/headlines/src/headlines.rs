use std::{borrowed::Cow, iter::FromIterator};

use eframe::egui::{self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel};

pub const PADDING: f32= 5.0;

pub struct Headlines{
    articles: Vec<NewsCardData>
}

struct NewsCardData{
    title: String,
    desc: String,
    url: String,
}


impl Headlines {
    pub fn new() -> Headlines{
        let iter= (0..20).map(|a| NewsCardData{
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a)
        });
        Headlines{
            articles: Vec::from_iter(iter)
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef){
        let mut font_def= FontDefinitions::default();
        font_def.font_data.insert("MsloLGS".to_string(), Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regula.ttf")));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 35));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20));
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0,"MsloLGS".to_string());
    
        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::UI){
        for a in &self.articles{
            ui.add_space(PADDING);

            // render
            let title= format!("NEXT PLAY {}", a.title);
            ui.colored_labal(WHITE, title);

            ui.add_space(PADDING);
            let desc= Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            ui.style_mut().visuals.hyperlink_color= CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("read more @return"));
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &CtxRef){
        TopBottomPanel::top("top_panel").show(ctx, |ui|{
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // Logo
                ui.with_layout(Layout::letf_to_right(), |ui|{
                    ui.add(Label::new("IMOZI").text_style(egui::TextStyle::Heading));
                });

                // Controls
                ui.with_layout(Layout::right_to_left(), |ui|{
                    let close_btn= ui.add(Button::new("X").text_style(egui::TextStyle::Body));
                    let refresh_btn= ui.add(Button::new("Refresh").text_style(egui::TextStyle::Body));
                    let theme_btn= ui.add(Button::new("Dark").text_style(egui::TextStyle::Body));
                });
            });
            ui.add_space(10.);
        });
    }
}
