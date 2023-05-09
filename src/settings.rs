use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Settings {
    pub project_name: Option<String>,
}

/*
fernflower:
rbr (1): hide bridge methods
rsy (0): hide synthetic class members
din (1): decompile inner classes
dc4 (1): collapse 1.4 class references
das (1): decompile assertions
hes (1): hide empty super invocation
hdc (1): hide empty default constructor
dgs (0): decompile generic signatures
ner (1): assume return not throwing exceptions
den (1): decompile enumerations
rgn (1): remove getClass() invocation, when it is part of a qualified new statement
lit (0): output numeric literals "as-is"
asc (0): encode non-ASCII characters in string and character literals as Unicode escapes
bto (1): interpret int 1 as boolean true (workaround to a compiler bug)
nns (0): allow for not set synthetic attribute (workaround to a compiler bug)
uto (1): consider nameless types as java.lang.Object (workaround to a compiler architecture flaw)
udv (1): reconstruct variable names from debug information, if present
ump (1): reconstruct parameter names from corresponding attributes, if present
rer (1): remove empty exception ranges
fdi (1): de-inline finally structures
mpm (0): maximum allowed processing time per decompiled method, in seconds. 0 means no upper limit
ren (0): rename ambiguous (resp. obfuscated) classes and class elements
urc (-): full name of a user-supplied class implementing IIdentifierRenamer interface. It is used to determine which class identifiers should be renamed and provides new identifier names (see "Renaming identifiers")
inn (1): check for IntelliJ IDEA-specific @NotNull annotation and remove inserted code if found
lac (0): decompile lambda expressions to anonymous classes
nls (0): define new line character to be used for output. 0 - '\r\n' (Windows), 1 - '\n' (Unix), default is OS-dependent
ind: indentation string (default is 3 spaces)
log (INFO): a logging level, possible values are TRACE, INFO, WARN, ERROR
 */
