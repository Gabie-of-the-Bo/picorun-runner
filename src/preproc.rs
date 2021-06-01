pub fn preproc_python_code(code: &str) -> String{
    let prep_import = "from preparation import *
from typing import *";

    return format!("{}\n\n{}", prep_import, code);
}

pub fn preproc_python_execution(code: &str) -> String{
    let imports = "from preparation import *
from code import *
from utils import *";

    return format!("{}\n\n{}", imports, code);
}