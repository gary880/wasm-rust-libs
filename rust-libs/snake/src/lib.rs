use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{self, Stylize},
    terminal::{self, ClearType},
    QueueableCommand,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}
#[derive(Debug)]
struct Snake {
    body: Vec<Point>,
    dir: Direction,
}

impl Snake {
    pub fn init() -> Self {
        let body = vec![
            Point { x: 10, y: 8 }, // 這是頭 (index 0)
            Point { x: 9, y: 8 },  // 這是脖子
            Point { x: 8, y: 8 },  // 這是尾巴
        ];
        let dir = Direction::Right;
        Snake { body, dir }
    }

    pub fn next_head_point(&self) -> Point {
        let head = self.body[0];
        match self.dir {
            Direction::Up => Point {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        }
    }

    pub fn move_forward(&mut self, grow: bool) {
        let new_head = self.next_head_point();
        self.body.insert(0, new_head);
        if !grow {
            self.body.pop();
        }
    }
}

#[derive(Debug)]
struct Game {
    snake: Snake,
    food: Point,
    score: u32,
    width: u16,
    height: u16,
    is_over: bool,
}

impl Game {
    pub fn init(width: u16, height: u16) -> Self {
        let snake = Snake::init();
        let mut game = Game {
            snake,
            food: Point { x: 0, y: 0 },
            score: 0,
            width,
            height,
            is_over: false,
        };
        game.spawn_food();
        game
    }

    pub fn render(&self) -> std::io::Result<()> {
        let mut stdout = stdout();

        stdout.queue(terminal::Clear(ClearType::All))?;
        stdout.queue(cursor::Hide)?;

        stdout
            .queue(cursor::MoveTo(self.food.x, self.food.y))?
            .queue(style::PrintStyledContent("*".yellow()))?;

        // 繪製上下牆 (從 0 畫到 width)
        for x in 0..=self.width {
            stdout
                .queue(cursor::MoveTo(x, 0))?
                .queue(style::Print("#".dark_grey()))?;
            stdout
                .queue(cursor::MoveTo(x, self.height))?
                .queue(style::Print("#".dark_grey()))?;
        }
        // 繪製左右牆 (從 0 畫到 height)
        for y in 0..=self.height {
            stdout
                .queue(cursor::MoveTo(0, y))?
                .queue(style::Print("#".dark_grey()))?;
            stdout
                .queue(cursor::MoveTo(self.width, y))?
                .queue(style::Print("#".dark_grey()))?;
        }
        for (i, part) in self.snake.body.iter().enumerate() {
            stdout.queue(cursor::MoveTo(part.x, part.y))?;
            if i == 0 {
                stdout.queue(style::PrintStyledContent("*".green().bold()))?;
            } else {
                stdout.queue(style::PrintStyledContent("●".green().bold()))?;
            }
        }

        stdout.queue(cursor::MoveTo(0, self.height + 3))?;
        println!("Score: {}", self.score);

        stdout.flush()?;

        Ok(())
    }
    pub fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Point {
                x: rng.gen_range(1..self.width),
                y: rng.gen_range(1..self.height),
            };
            if !self.snake.body.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    pub fn update(&mut self) {
        if self.is_over {
            return;
        }

        let next_head = self.snake.next_head_point();

        // 1. 邊界碰撞判定
        // 牆壁佔據了 0 和 width/height 座標
        // 因此蛇頭的可行走區域應該在 1..(width-1) 之間
        if next_head.x <= 0
            || next_head.x >= self.width
            || next_head.y <= 0
            || next_head.y >= self.height
        {
            self.is_over = true;
            return;
        }

        // 2. 自我碰撞判定 (關鍵！)
        // 檢查新頭部座標是否已經存在於目前的身體 Vec 中
        if self.snake.body.contains(&next_head) {
            self.is_over = true;
            return;
        }

        // 3. 食物與移動邏輯
        if next_head == self.food {
            self.score += 1;
            self.snake.move_forward(true);
            self.spawn_food();
        } else {
            self.snake.move_forward(false);
        }
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::init(20, 20);
    let frame_duration = Duration::from_millis(150);

    loop {
        let start_time = Instant::now();

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up if game.snake.dir != Direction::Down => {
                            game.snake.dir = Direction::Up;
                        }
                        KeyCode::Down if game.snake.dir != Direction::Up => {
                            game.snake.dir = Direction::Down;
                        }
                        KeyCode::Left if game.snake.dir != Direction::Right => {
                            game.snake.dir = Direction::Left;
                        }
                        KeyCode::Right if game.snake.dir != Direction::Left => {
                            game.snake.dir = Direction::Right;
                        }
                        // 允許按下 Esc 鍵退出遊戲
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        game.update();
        game.render()?;

        if game.is_over {
            break;
        }

        let elapsed = start_time.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    println!("game over scored {}", game.score);
    Ok(())
}
