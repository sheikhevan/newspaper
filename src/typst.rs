use chrono::Local;
use serde::Serialize;
use std::fs;
use std::process::Command;
use tera::{Context, Tera};

#[derive(Serialize, Debug)]
pub struct NewsEntry {
    pub title: String,
    pub author: String,
    pub content: String,
    pub link: String,
    pub pub_date: String,
}

#[derive(Serialize, Debug)]
pub struct Newspaper {
    pub date: String,
    pub articles: Vec<NewsEntry>,
    pub total_articles: usize,
}

impl Newspaper {
    pub fn new() -> Self {
        Self {
            date: Local::now().format("%A, %B %d, %Y").to_string(),
            articles: Vec::new(),
            total_articles: 0,
        }
    }

    pub fn add_article(
        &mut self,
        title: String,
        author: String,
        content: String,
        link: String,
        pub_date: String,
    ) {
        self.articles.push(NewsEntry {
            title,
            author,
            content,
            link,
            pub_date,
        });
        self.total_articles = self.articles.len();
    }
}

pub struct TypstRenderer {
    tera: Tera,
}

impl TypstRenderer {
    pub fn new(templates_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut tera = Tera::new(&format!("{}/**/*", templates_dir))?;
        tera.autoescape_on(vec![]);

        Ok(Self { tera })
    }

    fn render_newspaper(
        &self,
        newspaper: &Newspaper,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut context = Context::new();
        context.insert("newspaper", newspaper);

        let rendered = self.tera.render("newspaper.typ", &context)?;
        Ok(rendered)
    }

    pub fn generate_pdf(
        &self,
        newspaper: &Newspaper,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let typst_content = self.render_newspaper(newspaper)?;

        let tmp_path = "tmp.typ";
        fs::write(tmp_path, typst_content)?;

        let output = Command::new("typst")
            .arg("compile")
            .arg(tmp_path)
            .arg(output_path)
            .output()?;

        let _ = fs::remove_file(tmp_path);

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Compilation to pdf with typst failed: {}", error).into());
        }

        println!(
            "Newspaper generated successfully. It can be found at {}",
            output_path
        );
        Ok(())
    }
}
