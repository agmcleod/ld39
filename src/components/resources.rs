use components::GathererType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Oil,
    Clean,
}

impl ResourceType {
    fn get_current_amount(&self, resources: &Resources) -> usize {
        resources.coal + resources.oil + resources.clean
    }
}

pub struct Resources {
    pub coal: usize,
    pub oil: usize,
    pub clean: usize,
    pub current_type: ResourceType,
}

impl Resources {
    pub fn new() -> Resources {
        Resources{
            coal: 0,
            oil: 0,
            clean: 0,
            current_type: ResourceType::Coal,
        }
    }

    // TODO: Consider overflow
    pub fn withdraw_all_for_type(&mut self, resource_type: ResourceType) -> usize {
        match resource_type {
            ResourceType::Coal => {
                let amount = self.coal;
                self.coal = 0;
                amount
            },
            ResourceType::Oil => {
                let amount = self.oil;
                self.oil = 0;
                amount
            },
            ResourceType::Clean => {
                let amount = self.clean;
                self.clean = 0;
                amount
            },
        }
    }

    pub fn get_amount(&mut self) -> usize {
        self.current_type.get_current_amount(self)
    }

    pub fn get_amount_for_type(&self, resource_type: &ResourceType) -> usize {
        match *resource_type {
            ResourceType::Coal => self.coal,
            ResourceType::Oil => self.oil,
            ResourceType::Clean => self.clean,
        }
    }

    pub fn get_current_type(&self) -> ResourceType {
        self.current_type
    }

    pub fn increase_type_for_gatherer_type(&mut self, gatherer_type: &GathererType) {
        match *gatherer_type {
            GathererType::Coal => {
                self.coal += 1;
            },
            GathererType::Oil => {
                self.oil += 2;
            },
            GathererType::Clean => {
                self.clean += 4;
            },
        }
    }

    pub fn reset(&mut self) {
        self.coal = 50;
        self.oil = 0;
        self.clean = 0;
        self.current_type = ResourceType::Coal;
    }
}
