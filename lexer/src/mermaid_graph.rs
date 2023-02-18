use std::process::Command;
use std::io::Write;

pub trait MermaidGraph {
    fn generate_html(diagram_header: &str, diagram_content: &str) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8" /></head>
  <body>
    <pre class="mermaid">
      {diagram_header}{diagram_content}
    </pre>
    <script type="module">
      import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@9/dist/mermaid.esm.min.mjs';
      mermaid.initialize({{ startOnLoad: true }});
    </script>
  </body>
</html>
        "#)
    }

    fn write_to_file(file: &str, content: &str) {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file)
            .unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
    }

    fn open_file(file: &str) {
        if cfg!(unix) {
            Command::new("open")
                .arg(file)
                .spawn()
                .unwrap();
        } else {
            Command::new("cmd")
                .arg("/c")
                .arg("start")
                .arg(file)
                .spawn()
                .unwrap();
        }
    }

    fn header(&self) -> &'static str;

    fn generate_and_open_graph(&self, file: &str) -> String {
        let html = Self::generate_html(self.header(), self.get_mermaid_content());
        Self::write_to_file(file, &html);
        Self::open_file(file);

        html
    }

    fn get_mermaid_content(&self) -> &str;
}
