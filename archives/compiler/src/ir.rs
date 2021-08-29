/// アーキテクチャに依存しない中間表現
#[derive(Debug, Default)]
pub struct Ir {
    /// グローバル変数の個数
    pub num_globals: i32,
    /// 文字列プール
    pub string_pool: Vec<String>,
    /// 命令列
    pub insts: Vec<IrInst>,
}

/// 中間表現で用いられる命令
#[derive(Debug)]
pub enum IrInst {
    /// 文字列プールから指定した文字列のアドレスを取得する
    GetStaticStr(i32),
    /// 指定したグローバル変数の値を取得してスタックに積む
    GetGlobal(i32),
    /// 指定したグローバル変数に、スタックからポップした値を代入する
    SetGlobal(i32),
    /// スタックからポップした値を出力する
    Print,
}
