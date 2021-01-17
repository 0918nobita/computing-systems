pub struct DataSectionItem {
    pub name: String,
    pub size: String,
    pub values: String,
}

pub struct DataSection {
    pub items: Vec<DataSectionItem>,
}

pub enum TextSectionItem {
    Label(String),
    Instruction(String),
}

pub struct TextSection {
    pub items: Vec<TextSectionItem>,
}

pub struct Asm {
    pub data: DataSection,
    pub text: TextSection,
}

impl Asm {
    pub fn stringify(&self) -> String {
        let mut result = String::from("bits 64\nglobal _start\n\nsection .data\n");
        for item in self.data.items.iter() {
            result.push_str(format!("    {} {} {}\n", item.name, item.size, item.values).as_str());
        }
        result.push_str("\nsection .text\n");
        for item in self.text.items.iter() {
            match item {
                TextSectionItem::Label(label_name) => {
                    result.push_str(format!("{}:\n", label_name).as_str());
                }
                TextSectionItem::Instruction(inst) => {
                    result.push_str(format!("    {}\n", inst).as_str());
                }
            }
        }
        result
    }
}
