use crate::app::{KeyEventHandler, Renderable};
use crate::dto::Torrent;
use crate::mapper::Mapper;
use crate::service::Service;
use crate::ui::view::view_key_bindings::KeyBindingView;
use crate::ui::view::view_torrent::TorrentView;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect, Size};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, HighlightSpacing, LineGauge, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget, Wrap};
use ratatui::{symbols, Frame};
use std::ops::Add;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::{Fill, Length, Min, Percentage};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use tui_scrollview::{ScrollView, ScrollViewState};

pub struct InfoScreen {
    service: Service,
    mapper: Mapper,
    selected_row_index: usize,
    vertical_scroll_state: ScrollbarState,
    scroll_view_state: ScrollViewState,
    vertical_scroll: usize,
    text: [String; 3]
}

impl InfoScreen {

    pub fn new(service: Service, mapper: Mapper) -> Self {
        Self {
            service,
            mapper,
            selected_row_index: 0,
            vertical_scroll_state: ScrollbarState::default(),
            scroll_view_state: ScrollViewState::default(),
            vertical_scroll: 0,
            text: [String::new(), String::new(), String::new()]
        }
    }

    fn line_numbers(&self, height: u16) -> impl Widget {
        use std::fmt::Write;
        let line_numbers = (1..=height).fold(String::new(), |mut output, n| {
            let _ = writeln!(output, "{n:>4} ");
            output
        });
        Text::from(line_numbers).dim()
    }

    fn bars(&self) -> BarGroup<'static> {
        let CHART_DATA: [(&str, u64, Color); 3] = [
            ("Red", 2, Color::Red),
            ("Green", 7, Color::Green),
            ("Blue", 11, Color::Blue),
        ];
        let data = CHART_DATA
            .map(|(label, value, color)| Bar::default().label(label.into()).value(value).style(color));
        BarGroup::default().bars(&data)
    }

    fn vertical_bar_chart(&self) -> impl Widget {
        let block = Block::bordered().title("Vertical Bar Chart");
        BarChart::default()
            .direction(Direction::Vertical)
            .block(block)
            .bar_width(5)
            .bar_gap(1)
            .data(self.bars())
    }

    fn text(&self, index: usize) -> impl Widget {
        let block = Block::bordered().title(format!("Text {index}"));
        Paragraph::new(self.text[index].clone())
            .wrap(Wrap { trim: false })
            .block(block)
    }
}

impl Renderable for InfoScreen {
    fn render(&mut self, frame: &mut Frame, args: Vec<usize>) {
        self.selected_row_index = args[0];

        let torrents: Vec<Torrent> = self.mapper.json_to_response(self.service.torrent_list()).arguments.torrents;
        let torrent = torrents.get(self.selected_row_index).unwrap();

        let scroll_view_height = 30;
        let buf = frame.buffer_mut();

        let width = if buf.area.height < scroll_view_height {
            buf.area.width - 1
        } else {
            buf.area.width
        };
        let mut scroll_view = ScrollView::new(Size::new(width, scroll_view_height));
        let scroll_view_buf = scroll_view.buf_mut();

        let [gauge_area, info_area, peers_area, bottom_area] = Layout::vertical([
            Length(5),
            Min(1),
            Min(1),
            Length(1)
        ]).spacing(1)
        .vertical_margin(1)
        .horizontal_margin(1)
        .areas(scroll_view_buf.area);

        // gauge
        let mut block = Block::bordered()
            .title(Line::from(Span::from(" ").add(Span::from(torrent.name.clone()).bold().underlined()).add(Span::from(" "))))
            .padding(Padding::uniform(1));
        let gauge_style = Style::new().bg(Color::DarkGray).fg(Color::Gray).bold();
        LineGauge::default()
            .block(block)
            .filled_style(gauge_style)
            .line_set(symbols::line::NORMAL)
            .ratio(torrent.percentage_done()/100f64)
            .render(gauge_area, scroll_view_buf);

        // info
        let mut key_bindings = KeyBindingView::default();
        key_bindings
            .add(KeyBindingView::home())
            .add(KeyBindingView::quit());
        block = Block::bordered()
            .title(" Info ")
            .padding(Padding::uniform(1));
        let info = vec![
            Line::from("ETA: ".to_string().add(TorrentView::eta(torrent).as_str())),
            Line::from("Size: ".to_string().add(TorrentView::total_size(torrent).as_str())),
            Line::from("Added on: ".to_string().add(TorrentView::added_on(torrent).as_str())),
            Line::from("Download: ".to_string().add(TorrentView::download_rate(torrent).as_str())),
            Line::from("Upload: ".to_string().add(TorrentView::upload_rate(torrent).as_str())),
        ];
        Paragraph::new(info)
            .block(block)
            .render(info_area, scroll_view_buf);

        // peers
        block = Block::bordered()
            .title(" Peers ")
            .padding(Padding::uniform(1));
        List::new(vec![
                ListItem::from(format!("Connected to {} peers", torrent.peers_connected)),
                ListItem::from(format!("Getting from {} peers", torrent.peers_sending_to_us)),
                ListItem::from(format!("Sending to {} peers", torrent.peers_getting_from_us)),
            ]).block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
            .render(peers_area, scroll_view_buf);

        // bottom
        Line::from(key_bindings.items_as_line()).centered().render(bottom_area, scroll_view_buf);

        frame.render_stateful_widget(scroll_view, frame.area(), &mut self.scroll_view_state);
    }
}

impl KeyEventHandler for InfoScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent, event: Event) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll_view_state.scroll_down();
                    true
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll_view_state.scroll_up();
                    true
                }
                // leave
                KeyCode::Esc => {
                    false
                },
                // do not leave (maybe it will change in the future)
                _ => { true }
            }
        } else {
            false
        }
    }
}