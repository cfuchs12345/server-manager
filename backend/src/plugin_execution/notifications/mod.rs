use std::collections::HashMap;

use crate::{
    commands::CommandInput,
    common, datastore,
    models::{
        error::AppError,
        plugin::{
            data::DataDef,
            notification::{Notification, NotificationDef, Notifications},
            Plugin,
        },
    },
};

pub struct NotificationProcessor {
    map: HashMap<String, Notifications>,
}

impl NotificationProcessor {
    pub fn new() -> Self {
        NotificationProcessor {
            map: HashMap::new(),
        }
    }

    pub async fn finish(&self) -> Result<(), AppError> {
        let values: Vec<Notifications> = self.map.values().cloned().collect();
        log::debug!("notifications to persist: {:?}", values);
        for notifications in values {
            datastore::insert_or_update_notifications(notifications).await?;
        }
        Ok(())
    }

    pub fn is_relevant_data_for_processing(&self, data: &DataDef, plugin: &Plugin) -> bool {
        if plugin.notifications.is_empty() {
            return false;
        }

        plugin.notifications.iter().any(|n| n.data_id == data.id)
    }

    pub async fn process(
        &mut self,
        plugin: &Plugin,
        data: &DataDef,
        input_response_tuples: &[(CommandInput, String)],
    ) -> Result<(), AppError> {
        if !self.is_relevant_data_for_processing(data, plugin) {
            return Ok(());
        }

        let notification_defs_referencing_data: Vec<NotificationDef> = plugin
            .notifications
            .iter()
            .filter(|notification| notification.data_id == data.id)
            .cloned()
            .collect();

        for input_response_tuple in input_response_tuples {
            let input = &input_response_tuple.0;

            for notification_def in &notification_defs_referencing_data {
                log::trace!("notification_def {:?}", notification_def);

                if let Ok(result) =
                    common::script_match(&notification_def.script, &input_response_tuple.1)
                {
                    if result {
                        log::trace!(
                            "matched data for notification {} {}",
                            notification_def.data_id,
                            notification_def.id,
                        );
                        log::trace!("input {:?}", input);

                        if let Some(ipaddress) = input.get_ipaddress() {
                            let ipaddress = format!("{}", ipaddress);

                            let notification = Notification {
                                id: notification_def.id.clone(),
                                name: notification_def.name.clone(),
                                message: notification_def.message.clone(),
                                notification_level: notification_def.notification_level.clone(),
                            };

                            log::trace!("Created notification {:?}", notification);

                            self.map
                                .entry(ipaddress.clone())
                                .or_insert_with(|| Notifications {
                                    ipaddress: ipaddress.clone(),
                                    list: Vec::new(),
                                })
                                .list
                                .push(notification);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
