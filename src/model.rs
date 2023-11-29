//! Simplistic Model Layer
//! (With mock-store layer)
//! Not for use in production

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --- Ticket Types

// Clone - so we can send copies to the client
// Debug
// Serialize - so we can serialize this into JSON for sending to the client
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

// Deserialize - so we can deserialize from JSON into Rust
#[derive(Deserialize)]
// Body structure for ticket when using create API
pub struct TicketForCreate {
    pub title: String,
}

// endregion: --- Ticket Types

// region: --- Model Controller

#[derive(Clone)]
// Clone is to clone the Arc, not Vec
// Mutex then ensures exclusive access to the vector
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_for_create: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_for_create.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        // filters out None values
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller
