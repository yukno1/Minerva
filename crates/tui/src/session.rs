enum MessageType {
    UserInput,
    AgentOutput,
}

pub struct Message {
    message_type: MessageType,
    content: String,
}

static mut SESSION_ID: usize = 0;

pub struct Session {
    id: usize,
    history: Vec<Message>,
    length: usize,
}

impl Session {
    pub fn new() -> Self {
        unsafe {
            SESSION_ID += 1;
            Self {
                id: SESSION_ID,
                history: Vec::new(),
                length: 0,
            }
        }
    }

    pub fn add_userinput(&mut self, content: String) {
        self.history.push(Message {
            message_type: MessageType::UserInput,
            content,
        });
        self.length += 1;
    }

    pub fn assign_agentoutput_block(&mut self) {
        self.history.push(Message {
            message_type: MessageType::AgentOutput,
            content: "".to_string(),
        });
        self.length += 1;
    }

    pub fn push_agentoutput_chunk(&mut self, content: String) {
        self.history[self.length - 1]
            .content
            .push_str(content.as_str());
    }
}
