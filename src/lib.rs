use std::fmt::Debug;

#[derive(Debug)]
pub struct Id(u32);

#[derive(Debug)]
pub struct Ivc(usize);

#[derive(Debug)]
pub struct Ise(usize);

#[derive(Debug)]
pub struct Value(usize);

#[derive(Debug)]
pub struct CDHash(usize);

pub struct ExpandArrayFlags(u32);

impl ExpandArrayFlags {
    fn rest_args_array(&self) -> bool {
        self.0 & 0x1 != 0
    }

    fn post_arg(&self) -> bool {
        self.0 & 0x2 != 0
    }

    fn reverse(&self) -> bool {
        self.0 & 0x3 != 0
    }
}

impl Debug for ExpandArrayFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpandArrayFlags")
            .field("rest_args_array", &self.rest_args_array())
            .field("post_arg", &self.post_arg())
            .field("reverse?", &self.reverse())
            .finish()
    }
}

#[derive(Debug)]
pub enum BranchKind {
    If,
    Unless,
    Nil
}

#[derive(Debug)]
pub enum OpKind {
    NilP,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    LtLt,
    And,
    Or,
    ARef,
    ASet,
    Length,
    Size,
    EmptyP,
    Succ,
    Not,
    RegexpMatch2,
}

#[derive(Debug)]
pub enum OpWithKind {
    ARef,
    ASet
}

#[derive(Debug)]
pub enum JokeKind {
    Blt,
    Answer
}

#[derive(Debug)]
pub struct CallData();
#[derive(Debug)]
pub enum BuiltIn {}

pub enum Instruction {
    Nop,
    GetLocal {
        index: usize,
        level: usize,
    },
    SetLocal {
        index: usize,
        level: usize,
    },
    GetBlockParam {
        index: usize,
        level: usize,
    },
    SetBlockParam {
        index: usize,
        level: usize,
    },
    GetBlockParamProxy {
        index: usize,
        level: usize,
    },
    GetSpecial {
        key: u32,
        ty: u32,
    },
    SetSpecial {
        key: u32,
    },
    GetInstanceVariable {
        id: Id,
        ivc: Ivc,
    },
    SetInstanceVariable {
        id: Id,
        ivc: Ivc,
    },
    GetClassVariable {
        id: Id,
    },
    SetClassVariable {
        id: Id,
    },
    GetConstant {
        id: Id,
    },
    SetConstant {
        id: Id,
    },
    GetGlobal {
        id: Id,
    },
    SetGlobal {
        id: Id,
    },
    PutNil,
    PutSelf,
    PutObject {
        value: Value,
    },
    PutSpecialObject {
        value_type: u32,
    },
    PutString {
        string: u32,
    },
    ConcatStrings {
        num: usize,
    },
    ToString,
    ToRegexp,
    Intern,
    NewArray {
        num: usize,
    },
    NewArrayKwSplat {
        num: usize,
    },
    DupArray {
        ary: Value,
    },
    DupHash {
        hash: Value,
    },
    ExpandArray {
        num: usize,
        flags: ExpandArrayFlags,
    },
    ConcatArray,
    SplatArray {
        flag: Value,
    },
    NewHash {
        num: u32,
    },
    NewRange {
        flag: u32,
    },
    Pop,
    Dup,
    DupN {
        n: u32,
    },
    Swap,
    Reverse {
        n: u32,
    },
    RePut,
    TopN {
        n: u32,
    },
    SetN {
        n: u32,
    },
    AdjustStack {
        n: usize,
    },
    Defined {
        op: u32,
        obj: Value,
    },
    CheckMatch {
        flag: u32,
    },
    CheckKeyword {
        kw_bits_index: usize,
        kw_index: usize,
    },
    CheckType {
        ty: u32,
    },
    DefineClass {
        id: Id,
        iseq: Vec<Instruction>,
        flags: u32,
    },
    DefineMethod {
        id: Id,
        iseq: Vec<Instruction>,
    },
    DefineSMethod {
        id: Id,
        iseq: Vec<Instruction>,
    },
    Send {
        call_data: CallData,
        block_iseq: Vec<Instruction>,
    },
    OptSendWithoutBlock {
        call_data: CallData,
    },
    OptStrFreeze {
        string: Value,
        call_data: CallData,
    },
    OptStrUminus {
        string: Value,
        call_data: CallData,
    },
    OptNewArrayMax {
        num: u32,
    },
    OptNewArrayMin {
        num: u32,
    },
    InvokeSuper {
        call_data: CallData,
        block_iseq: Vec<Instruction>,
    },
    InvokeBlock {
        call_data: CallData,
    },
    Leave,
    Throw {
        throw_state: u32,
    },
    Jump {
        dst_offset: usize,
    },
    Branch {
	kind: BranchKind,
        dst_offset: usize,
    },
    OptSetInlineCache {
        inline_cache: u32,
    },
    Once {
        iseq: Vec<Instruction>,
        ise: Ise,
    },
    OptCaseDispatch {
	hash: CDHash,
	else_offset: usize
    },
    OptOp {
	kind: OpKind,
	call_data: CallData
    },
    OptOpWith {
	kind: OpWithKind,
	key: Value,
	all_data: u32,
    },
    OptCallCFunc {
	ptr: usize
    },
    InvokeBuiltin {
	builtin: BuiltIn
    },
    InvokeBuiltinDelegate {
	builtin: BuiltIn,
	index: usize,
	leave: bool
    },
    Joke {
	kind: JokeKind
    }
}
