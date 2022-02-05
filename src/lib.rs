#[macro_use]
extern crate lazy_static;
extern crate regex;


use regex::{Captures, Regex};

const PATTERNS_AMOUNT: usize = 30;

#[cfg(test)]
mod tests {
    use super::*;

    mod font_manipulation {
        use super::*;

        #[test]
        fn bold() {
            assert_eq!("[b]test[/b]".as_html(), "<strong>test</strong>")
        }

        #[test]
        fn italic() {
            assert_eq!("[i]test[/i]".as_html(), "<em>test</em>")
        }

        #[test]
        fn underline() {
            assert_eq!("[u]test[/u]".as_html(), r#"<u>test</u>"#)
        }

        #[test]
        fn strikethrough() {
            assert_eq!("[s]test[/s]".as_html(), r#"<strike>test</strike>"#)
        }

        #[test]
        fn color() {
            assert_eq!("[color=red]test[/color]".as_html(),
                       r#"<span style="color: red;">test</span>"#)
        }

        #[test]
        fn size() {
            assert_eq!("[size=10]test[/size]".as_html(),
                       r#"<span style="font-size: 10px;">test</span>"#)
        }
    }

    mod alignment {
        use super::*;

        #[test]
        fn left() {
            assert_eq!("[left]test[/left]".as_html(),
                       r#"<div style="text-align: left;">test</div>"#)
        }

        #[test]
        fn right() {
            assert_eq!("[right]test[/right]".as_html(),
                       r#"<div style="text-align: right;">test</div>"#)
        }

        #[test]
        fn center() {
            assert_eq!("[center]test[/center]".as_html(),
                       r#"<div style="text-align: center;">test</div>"#)
        }
    }

    #[test]
    fn quote() {
        assert_eq!("[quote]test[/quote]".as_html(),
                   r#"<blockquote>test</blockquote>"#)
    }

    #[test]
    fn named_quote() {
        assert_eq!("[quote=Author]test[/quote]".as_html(),
                   r#"<blockquote><strong>Author wrote:</strong>
test</blockquote>"#)
    }

    #[test]
    fn link() {
        assert_eq!("[url]test[/url]".as_html(),
                   r#"<a href="test" rel="nofollow" target="_new">test</a>"#)
    }

    #[test]
    fn named_link() {
        assert_eq!("[url=title]test[/url]".as_html(),
                   r#"<a href="test" rel="nofollow" target="_new">title</a>"#)
    }

    mod image {
        use super::*;

        #[test]
        fn plain() {
            assert_eq!("[img]test[/img]".as_html(), "<img src=\"test\" />")
        }

        #[test]
        fn named_image() {
            assert_eq!("[img=name]test[/img]".as_html(),
                       "<img src=\"test\" alt=\"name\" />")
        }

        #[test]
        fn resized_image() {
            assert_eq!("[img=100x50]test[/img]".as_html(),
                       "<img src=\"test\" width=\"100\" height=\"50\" />")
        }

        #[test]
        fn with_metadata() {
            assert_eq!(r#"[img width="100" height="50" alt="alt" title="title"]test[/img]"#.as_html(),
                       "<img src=\"test\" width=\"100\" height=\"50\" alt=\"alt\" \
                        title=\"title\" />")
        }
    }

    mod list {
        use super::*;

        #[test]
        fn ordered() {
            assert_eq!(r#"[ol]test[/ol]"#.as_html(), r#"<ol>test</ol>"#)
        }

        #[test]
        fn full_ordered_list() {
            assert_eq!(r#"[ol]
[li]Item one[/li]
[li]Item two[/li]
[/ol]"#.as_html(),
                       r#"<ol>
<li>Item one</li>
<li>Item two</li>
</ol>"#)
        }

        #[test]
        fn unordered() {
            assert_eq!(r#"[ul]test[/ul]"#.as_html(), r#"<ul>test</ul>"#)
        }

        #[test]
        fn full_unordered_list() {
            assert_eq!(r#"[ul]
[li]Item one[/li]
[li]Item two[/li]
[/ul]"#.as_html(),
                       r#"<ul>
<li>Item one</li>
<li>Item two</li>
</ul>"#)
        }

        #[test]
        fn plain() {
            assert_eq!(r#"[list]test[/list]"#.as_html(), r#"<ul>test</ul>"#)
        }
    }

    #[test]
    fn code() {
        assert_eq!("[code][b]test[/b][/code]".as_html(),
                   "<pre><code>&#91;b&#93;test&#91;/b&#93;</code></pre>")
    }

    #[test]
    fn preformatted() {
        assert_eq!("[preformatted][b]test[/b][/preformatted]".as_html(),
                   "<pre><code>&#91;b&#93;test&#91;/b&#93;</code></pre>")
    }

    mod table {
        use super::*;

        #[test]
        fn table() {
            assert_eq!("[table][/table]".as_html(), "<table></table>")
        }

        #[test]
        fn complex_table() {
            assert_eq!(r"[table]
              [tr]
                [th]Name[/th]
                [th]Date[/th]
              [/tr]
              [tr]
                [td]Test[/td]
                [td]Yesterday[/td]
              [/tr]
[/table]".as_html(),
                       r"<table>
              <tr>
                <th>Name</th>
                <th>Date</th>
              </tr>
              <tr>
                <td>Test</td>
                <td>Yesterday</td>
              </tr>
</table>")
        }

        #[test]
        fn row() {
            assert_eq!("[tr]test[/tr]".as_html(), "<tr>test</tr>")
        }

        mod content {
            use super::*;

            #[test]
            fn cell() {
                assert_eq!("[td]test[/td]".as_html(), "<td>test</td>")
            }

            #[test]
            fn header() {
                assert_eq!("[th]test[/th]".as_html(), "<th>test</th>")
            }
        }
    }

    #[test]
    fn youtube_video() {
        assert_eq!("[youtube]test[/youtube]".as_html(),
                   "<object data=\"http://www.youtube.com/embed/test\"></object>")
    }

    #[test]
    fn youtube_video_with_size() {
        assert_eq!("[youtube=560x315]test[/youtube]".as_html(),
                   "<object width=\"560\" \
                    height=\"315\" data=\"http://www.youtube.com/embed/test\"></object>")
    }

    #[test]
    fn sub() {
        assert_eq!("[sub]Test text[/sub]".as_html(),
                   "<sub>Test text</sub>")
    }

    #[test]
    fn sup() {
        assert_eq!("[sup]Test [/sup]text".as_html(),
                   "<sup>Test </sup>text")
    }
    #[test]
    fn gacha() {
        assert_eq!("[gacha=N]N: 青じそドレッシング[/gacha]text".as_html(),
                   "<span class=\"N\">N: 青じそドレッシング</span>text")
    }
    #[test]
    fn spin() {
        assert_eq!("[spin]spin[/spin]text".as_html(),
                   "<span class=\"fa fa-spin\">spin</span>text")
    }
    #[test]
    fn spin_spin() {
        assert_eq!("[spin]spin[spin]spin[/spin][/spin]text".as_html(),
                   "<span class=\"fa fa-spin\">spin<span class=\"fa fa-spin\">spin</span></span>text")
    }
}

/// BBCode is a trait that will convert the input BBCode into HTML
///
/// Included in this is a default ipml for &str, allowing you to
///
/// ```
/// use bbcode::BBCode;
///
/// assert_eq!("[b]Bold![/b]".as_html(), "<strong>Bold!</strong>");
/// ```
pub trait BBCode {
    fn as_html(&self) -> String;
}

fn code(input: &str) -> String {
    let mut output = input.to_owned();
    lazy_static! {
          static ref  CODE: Regex = Regex::new(
            r"(?s)\[(code|preformatted)\](.*?)\[/(code|preformatted)\]"
          ).unwrap();
        }

    output = CODE.replace_all(&output, code_replacer).to_string();
    output
}

fn code_replacer(captures: &Captures) -> String {
    let mut replaced = captures.get(2).unwrap().as_str().to_string();
    for &(input, output) in [("[", "&#91;"), ("]", "&#93;"), ("<br />", "\n")].iter() {
        replaced = replaced.replace(&input, &output);
    }
    format!("<pre><code>{}</code></pre>", replaced)
}

pub fn patterns() -> &'static [(Regex, &'static str); PATTERNS_AMOUNT] {
    lazy_static! {
        static ref  PATTERNS: [(Regex, &'static str); PATTERNS_AMOUNT] = [
          // Font changes
          (Regex::new(r"(?s)\[b\](.*?)\[/b\]").unwrap(), "<strong>$1</strong>"),
          (Regex::new(r"(?s)\[i\](.*?)\[/i\]").unwrap(), "<em>$1</em>"),
          (Regex::new(r"(?s)\[u\](.*?)\[/u\]").unwrap(), "<u>$1</u>"),
          (Regex::new(r"(?s)\[s\](.*?)\[/s\]").unwrap(), "<strike>$1</strike>"),
          (Regex::new(r"(?s)\[size=(\d+)](.*?)\[/size\]").unwrap(),
            "<span style=\"font-size: ${1}px;\">$2</span>"),
          (Regex::new(r"(?s)\[color=(.+)](.*?)\[/color\]").unwrap(),
            "<span style=\"color: $1;\">$2</span>"),
          // Alignment
          (Regex::new(r"(?s)\[center\](.*?)\[/center\]").unwrap(),
            "<div style=\"text-align: center;\">$1</div>"),
          (Regex::new(r"(?s)\[left\](.*?)\[/left\]").unwrap(),
            "<div style=\"text-align: left;\">$1</div>"),
          (Regex::new(r"(?s)\[right\](.*?)\[/right\]").unwrap(),
            "<div style=\"text-align: right;\">$1</div>"),
          // Tables
          (Regex::new(r"(?s)\[table\](.*?)\[/table\]").unwrap(), "<table>$1</table>"),
          (Regex::new(r"(?s)\[td\](.*?)\[/td\]").unwrap(), "<td>$1</td>"),
          (Regex::new(r"(?s)\[tr\](.*?)\[/tr\]").unwrap(), "<tr>$1</tr>"),
          (Regex::new(r"(?s)\[th\](.*?)\[/th\]").unwrap(), "<th>$1</th>"),
          // Links
          (Regex::new(r"(?s)\[url\](.*?)\[/url\]").unwrap(),
            "<a href=\"$1\" rel=\"nofollow\" target=\"_new\">$1</a>"),
          (Regex::new(r"(?s)\[url=(.+)\](.*?)\[/url\]").unwrap(),
            "<a href=\"$2\" rel=\"nofollow\" target=\"_new\">$1</a>"),
          // Quotes
          (Regex::new(r"(?s)\[quote\](.*?)\[/quote\]").unwrap(),
            "<blockquote>$1</blockquote>"),
          (Regex::new(r"(?s)\[quote=(.+)\](.*?)\[/quote\]").unwrap(),
            "<blockquote><strong>$1 wrote:</strong>\n$2</blockquote>"),
          // Images
          (Regex::new(r"(?s)\[img=(\d+)x(\d+)(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$4\" width=\"$1\" height=\"$2\"$3 />"),
          (Regex::new(r"(?s)\[img=(.+)(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$3\" alt=\"$1\"$2 />"),
          (Regex::new(r"(?s)\[img(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$2\"$1 />"),
          // Lists
          (Regex::new(r"(?s)\[ol\](.*?)\[/ol\]").unwrap(), "<ol>$1</ol>"),
          (Regex::new(r"(?s)\[ul\](.*?)\[/ul\]").unwrap(), "<ul>$1</ul>"),
          (Regex::new(r"(?s)\[list\](.*?)\[/list\]").unwrap(), "<ul>$1</ul>"),
          // Youtube
          (Regex::new(r"(?s)\[youtube\](.*?)\[/youtube\]").unwrap(),
            "<object data=\"http://www.youtube.com/embed/$1\"></object>"),
          (Regex::new(r"(?s)\[youtube=(\d+)x(\d+)\](.*?)\[/youtube\]").unwrap(),
          "<object width=\"$1\" height=\"$2\" data=\"http://www.youtube.com/embed/$3\"></object>"),
          // List Items
          (Regex::new(r"(?s)\[li\](.*?)\[/li\]").unwrap(), "<li>$1</li>"),
          (Regex::new(r"(?s)\[sub\](.*?)\[/sub\]").unwrap(), "<sub>$1</sub>"),
          (Regex::new(r"(?s)\[sup\](.*?)\[/sup\]").unwrap(), "<sup>$1</sup>"),
          (Regex::new(r"(?s)\[gacha=(N|HN|R|SR|SSR|UR|LR)\](.*?)\[/gacha\]").unwrap(), "<span class=\"$1\">$2</span>"),
            (Regex::new(r"(?s)\[spin](.*?)\[/spin\]").unwrap(), "<span class=\"fa fa-spin\">$1</span>"),
          ];

      }
    &PATTERNS
}

pub fn str_to_html(input: &str) -> String {
    let mut output = code(&input);
    for &(ref pattern, replace) in patterns() {
        loop {
            match pattern.is_match(&output) {
                true => {
                    output = pattern.replace_all(&output, replace).into_owned();
                },
                false => {
                    break
                }
            }
        }
    }
    output
}

impl<'a> BBCode for &'a str {
    fn as_html(&self) -> String {
        str_to_html(&self)
    }
}
