use std::io::{self, Write};
use std::sync::mpsc;
use std::{thread, time};

pub struct Animation {
    tx: std::sync::mpsc::Sender<bool>,
    handle: std::option::Option<std::thread::JoinHandle<()>>,
}

impl Animation {
    pub fn new(str: std::string::String) -> Self {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let msgs = vec![
                "[...              ]",
                "[ ...             ]",
                "[  ...            ]",
                "[   ...           ]",
                "[    ...          ]",
                "[     ...         ]",
                "[      ...        ]",
                "[       ...       ]",
                "[        ...      ]",
                "[         ...     ]",
                "[          ...    ]",
                "[           ...   ]",
                "[            ...  ]",
                "[             ... ]",
                "[              ...]",
                "[             ... ]",
                "[            ...  ]",
                "[           ...   ]",
                "[          ...    ]",
                "[         ...     ]",
                "[        ...      ]",
                "[       ...       ]",
                "[      ...        ]",
                "[     ...         ]",
                "[    ...          ]",
                "[   ...           ]",
                "[  ...            ]",
                "[ ...             ]",
            ];

            println!("{}", str);
            'outer: loop {
                for msg in msgs.iter() {
                    print!("{}\r", msg);
                    io::stdout().flush().unwrap();
                    thread::sleep(time::Duration::from_millis(50));
                    match rx.try_recv() {
                        Ok(_) => {
                            print!("\r");
                            io::stdout().flush().unwrap();
                            break 'outer;
                        }
                        Err(_e) => continue,
                    }
                }
            }
        });
        Self {
            tx: tx,
            handle: Some(handle),
        }
    }

    pub fn end(&mut self) -> () {
        self.tx.send(true).unwrap();
        if let Some(h) = self.handle.take() {
            h.join().unwrap();
        }
        print!("\r");
        std::io::stdout().flush().unwrap();
    }
}
