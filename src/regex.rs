use regex_lite::Regex;

// Lazily initialize a global variable.
#[doc(hidden)]
#[macro_export]
macro_rules! lazy_static {
    ($f:ident, $t:ty, $e:block $(;)?) => {
        lazy_static! { $f -> &'static $t, $t, $e }
    };
    ($f:ident -> $ret:ty, $t:ty, $e:block $(;)?) => {
        #[allow(dead_code)]
        fn $f() -> $ret {
            static TMP: ::std::sync::OnceLock<$t> = ::std::sync::OnceLock::new();
            TMP.get_or_init(|| $e)
        }
    };
    ($( $f:ident, $t:ty, $e:block );+ $(;)?) => {
        $( lazy_static! { $f, $t, $e } )+
    };
    ($( $f:ident -> $ret:ty, $t:ty, $e:block );+ $(;)?) => {
        $( lazy_static! { $f -> $ret, $t, $e } )+
    };
}

const RE_ERROR: &str = "regex pattern error";

lazy_static! {
    re, Re, {
        Re {
            // running 0 tests; running 1 test; running 2 tests; ...
            head: Regex::new(r"running \d+ tests?").expect(RE_ERROR),
            // Common test info:
            // test submod::normal_test ... ok
            // test submod::ignore ... ignored, reason
            // test submod::ignore_without_reason ... ignored
            // test submod::panic::should_panic - should panic ... ok
            // test submod::panic::should_panic_without_reanson - should panic ... ok
            tree: Regex::new(r"(?m)^test \S+( - should panic)? \.\.\. \S+(, .*)?$").expect(RE_ERROR)
        }
    };
}

pub struct ParsedCargoTestOutput<'s> {
    pub head: &'s str,
    pub tree: Vec<&'s str>,
    pub detail: &'s str,
}

pub fn parse_cargo_test_output(text: &str) -> ParsedCargoTestOutput<'_> {
    let head = re()
        .head
        .find(text)
        .expect("`running \\d+ tests` not found");
    let head_end = head.end() + 1;
    let line: Vec<_> = re().tree.find_iter(&text[head_end..]).collect();
    let tree_end = line.last().map_or(head_end, |cap| head_end + cap.end() + 1);
    let mut tree: Vec<_> = line.into_iter().map(|cap| cap.as_str()).collect();
    tree.sort_unstable();
    ParsedCargoTestOutput {
        head: head.as_str(),
        tree,
        detail: text[tree_end..].trim(),
    }
}
