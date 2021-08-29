pub enum DataSectionItem {
    Data {
        name: String,
        size: String,
        values: String,
    },
    DefineMacro {
        name: String,
        replacement: String,
    },
    StrlenMacro {
        name: String,
        str: String,
    },
}

/// データセクションの内部表現
#[derive(Default)]
pub struct DataSection {
    items: Vec<DataSectionItem>,
}

impl DataSection {
    pub fn append<N, S, V>(&mut self, name: N, size: S, values: V)
    where
        N: Into<String>,
        S: Into<String>,
        V: Into<String>,
    {
        self.items.push(DataSectionItem::Data {
            name: name.into(),
            size: size.into(),
            values: values.into(),
        });
    }

    pub fn define<N, R>(&mut self, name: N, replacement: R)
    where
        N: Into<String>,
        R: Into<String>,
    {
        self.items.push(DataSectionItem::DefineMacro {
            name: name.into(),
            replacement: replacement.into(),
        });
    }

    pub fn strlen<N, S>(&mut self, name: N, str: S)
    where
        N: Into<String>,
        S: Into<String>,
    {
        self.items.push(DataSectionItem::StrlenMacro {
            name: name.into(),
            str: str.into(),
        });
    }
}

pub enum TextSectionItem {
    Label(String),
    Instruction(String),
}

/// テキストセクションの内部表現
#[derive(Default)]
pub struct TextSection {
    items: Vec<TextSectionItem>,
}

impl TextSection {
    pub fn label<N: Into<String>>(&mut self, name: N) {
        self.items.push(TextSectionItem::Label(name.into()));
    }

    pub fn inst<I: Into<String>>(&mut self, inst: I) {
        self.items.push(TextSectionItem::Instruction(inst.into()));
    }

    pub fn extend(&mut self, other: TextSection) {
        self.items.extend(other.items)
    }
}

/// アセンブリの内部表現
pub struct Asm {
    pub data: DataSection,
    pub text: TextSection,
}

impl Asm {
    /// 出力の ``.s`` (アセンブリ) ファイルに書き込まれる文字列を生成する
    pub fn stringify(&self) -> String {
        let mut result = String::from("bits 64\n");
        result.push_str("global _start\n\n");
        result.push_str("%define ERR_MSG 'Type Error', 0\n");
        result.push_str("%strlen ERR_MSG_CNT ERR_MSG\n\n");
        result.push_str("%define EXIT_FAILURE 1\n\n");
        result.push_str("%define FD_STDOUT 1\n");
        result.push_str("%define FD_STDERR 2\n\n");
        result.push_str("%define SYS_EXIT 60\n");
        result.push_str("%define SYS_WRITE 1\n\n");
        result.push_str("%define TYPE_STR 1\n\n");
        result.push_str("section .data\n");
        result.push_str("    err_msg db ERR_MSG\n");

        for item in self.data.items.iter() {
            match item {
                DataSectionItem::Data { name, size, values } => {
                    result.push_str(format!("    {} {} {}\n", name, size, values).as_str());
                }
                DataSectionItem::DefineMacro { name, replacement } => {
                    result.push_str(format!("    %define {} {}\n", name, replacement).as_str());
                }
                DataSectionItem::StrlenMacro { name, str } => {
                    result.push_str(format!("    %strlen {} {}\n", name, str).as_str());
                }
            }
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
