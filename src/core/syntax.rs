use serde::{Deserialize, Serialize};

/// Indicates which highlight should be applied to a region of source code.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Highlight(pub usize);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LanguageConfiguration {
    #[serde(rename = "name")]
    pub language_id: String, //c-sharp, rust
}

/// Represents a single step in rendering a syntax-highlighted document.
#[derive(Copy, Clone, Debug)]
pub enum HighlightEvent {
    Source { start: usize, end: usize },
    HighlightStart(Highlight),
    HighlightEnd,
}

pub struct Merge<I> {
    iter: I,
    spans: Box<dyn Iterator<Item = (usize, std::ops::Range<usize>)>>,

    next_event: Option<HighlightEvent>,
    next_span: Option<(usize, std::ops::Range<usize>)>,

    queue: Vec<HighlightEvent>,
}

pub fn merge<I: Iterator<Item = HighlightEvent>>(iter: I, spans: Vec<(usize, std::ops::Range<usize>)>) -> Merge<I> {
    let spans = Box::new(spans.into_iter());
    let mut merge = Merge {
        iter,
        spans,
        next_event: None,
        next_span: None,
        queue: Vec::new(),
    };
    merge.next_event = merge.iter.next();
    merge.next_span = merge.spans.next();
    merge
}

impl<I: Iterator<Item = HighlightEvent>> Iterator for Merge<I> {
    type Item = HighlightEvent;

    fn next(&mut self) -> Option<Self::Item> {
        use HighlightEvent::*;
        if let Some(event) = self.queue.pop() {
            return Some(event);
        }

        // TODO: handle range if offscreen case

        match (self.next_event, &self.next_span) {
            (Some(Source { start, end }), Some((_, range))) if start < range.start => {
                let intersect = range.start.min(end);

                let event = Source { start, end: intersect };

                if end == intersect {
                    self.next_event = self.iter.next();
                } else {
                    self.next_event = Some(Source { start: intersect, end });
                };

                Some(event)
            }
            (Some(Source { start, end }), Some((span, range))) if start == range.start => {
                let intersect = range.end.min(end);
                let event = HighlightStart(Highlight(*span));

                // enqueue in reverse order
                self.queue.push(HighlightEnd);
                self.queue.push(Source { start, end: intersect });

                if end == intersect {
                    // the event is complete
                    self.next_event = self.iter.next();
                } else {
                    self.next_event = Some(Source { start: intersect, end })
                };

                if intersect == range.end {
                    self.next_span = self.spans.next();
                } else {
                    self.next_span = Some((*span, intersect..range.end));
                };

                Some(event)
            }
            (Some(event), None) => {
                self.next_event = self.iter.next();
                Some(event)
            }
            (None, None) => None,
            e => todo!("Merge::next() {e:?}"),
        }
    }
}
