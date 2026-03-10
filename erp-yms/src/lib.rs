use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppointmentStatus {
    Scheduled,
    Arrived,
    Docked,
    Departed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YardAppointment {
    pub id: Uuid,
    pub vehicle_license_plate: String,
    pub driver_name: String,
    pub appointment_time: DateTime<Utc>,
    pub status: AppointmentStatus,
    pub dock_door: Option<String>,
}

impl YardAppointment {
    pub fn new(vehicle_license_plate: String, driver_name: String, appointment_time: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            vehicle_license_plate,
            driver_name,
            appointment_time,
            status: AppointmentStatus::Scheduled,
            dock_door: None,
        }
    }

    pub fn mark_arrived(&mut self) -> Result<(), &'static str> {
        if self.status != AppointmentStatus::Scheduled {
            return Err("Cannot mark as arrived unless scheduled");
        }
        self.status = AppointmentStatus::Arrived;
        Ok(())
    }

    pub fn assign_dock(&mut self, dock_door: String) -> Result<(), &'static str> {
        if self.status != AppointmentStatus::Arrived {
            return Err("Cannot assign dock unless arrived");
        }
        self.dock_door = Some(dock_door);
        self.status = AppointmentStatus::Docked;
        Ok(())
    }

    pub fn mark_departed(&mut self) -> Result<(), &'static str> {
        if self.status != AppointmentStatus::Docked {
            return Err("Cannot mark as departed unless docked");
        }
        self.dock_door = None;
        self.status = AppointmentStatus::Departed;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), &'static str> {
        if self.status == AppointmentStatus::Departed {
            return Err("Cannot cancel a departed appointment");
        }
        self.status = AppointmentStatus::Cancelled;
        self.dock_door = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_yard_appointment_lifecycle() {
        let appointment_time = Utc.with_ymd_and_hms(2026, 3, 10, 14, 0, 0).unwrap();
        let mut appt = YardAppointment::new(
            "ABC-1234".to_string(),
            "John Doe".to_string(),
            appointment_time,
        );

        assert_eq!(appt.status, AppointmentStatus::Scheduled);
        assert_eq!(appt.dock_door, None);

        // Mark as arrived
        assert!(appt.mark_arrived().is_ok());
        assert_eq!(appt.status, AppointmentStatus::Arrived);

        // Assign dock
        assert!(appt.assign_dock("Door-4".to_string()).is_ok());
        assert_eq!(appt.status, AppointmentStatus::Docked);
        assert_eq!(appt.dock_door, Some("Door-4".to_string()));

        // Mark as departed
        assert!(appt.mark_departed().is_ok());
        assert_eq!(appt.status, AppointmentStatus::Departed);
        assert_eq!(appt.dock_door, None);
    }

    #[test]
    fn test_invalid_state_transitions() {
        let appointment_time = Utc::now();
        let mut appt = YardAppointment::new(
            "XYZ-9876".to_string(),
            "Jane Smith".to_string(),
            appointment_time,
        );

        // Cannot assign dock if just scheduled
        assert!(appt.assign_dock("Door-1".to_string()).is_err());
        
        // Cannot mark departed if just scheduled
        assert!(appt.mark_departed().is_err());

        // Cancel the appointment
        assert!(appt.cancel().is_ok());
        assert_eq!(appt.status, AppointmentStatus::Cancelled);

        // Cannot mark arrived if cancelled
        assert!(appt.mark_arrived().is_err());
    }
}
