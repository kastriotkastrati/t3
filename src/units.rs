pub trait Inner {
    type InnerType;
    fn inner(&self) -> Self::InnerType;
}

#[derive(Debug, Clone)]
pub enum LengthUnit {
    Km(f64),
    M(f64),
}

impl LengthUnit {
    pub fn inner(&self) -> f64 {
        return match self {
            Self::Km(km) => km.clone(),
            Self::M(m) => m.clone(),
        };
    }

    pub fn km(&self) -> Self {
        return match self {
            Self::Km(km) => Self::Km(km.clone()),
            Self::M(m) => Self::Km(m / 1000.0),
        };
    }

    pub fn m(&self) -> Self {
        return match self {
            Self::Km(km) => Self::M(km * 1000.0),
            Self::M(m) => Self::M(m.clone()),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Degrees(pub f64);

#[derive(Debug, Clone)]
pub struct Radians(pub f64);

#[derive(Debug, Clone)]
pub enum AngleUnit {
    Degrees(Degrees),
    Radians(Radians),
}

impl Inner for Degrees {
    type InnerType = f64;
    fn inner(&self) -> Self::InnerType {
        return self.0;
    }
}

impl Inner for Radians {
    type InnerType = f64;
    fn inner(&self) -> Self::InnerType {
        return self.0;
    }
}

impl AngleUnit {
    pub fn value(&self) -> f64 {
        return match &self {
            AngleUnit::Degrees(x) => x.inner(),
            AngleUnit::Radians(x) => x.inner(),
        };
    }

    pub fn as_degrees(&self) -> Degrees {
        return match &self {
            AngleUnit::Degrees(x) => {
                let degrees_value = x.inner();
                let inner = Degrees(degrees_value);
                inner
            }
            _ => {
                let inner = self.radian_to_degrees().expect("Impossible case");
                inner
            }
        };
    }

    pub fn as_radians(&self) -> Radians {
        return match &self {
            AngleUnit::Radians(x) => {
                let radians_value = x.inner();
                let inner = Radians(radians_value);
                inner
            }
            _ => {
                let inner = self.degrees_to_radian().expect("Impossible case");
                inner
            }
        };
    }

    pub fn radian_to_degrees(&self) -> Result<Degrees, &str> {
        return match &self {
            AngleUnit::Radians(x) => {
                let degrees_value = x.inner().to_degrees();
                let inner = Degrees(degrees_value);
                Ok(inner)
            }
            _ => Err("Data is not in radian"),
        };
    }

    pub fn degrees_to_radian(&self) -> Result<Radians, &str> {
        return match &self {
            AngleUnit::Degrees(x) => {
                let radians_value = x.inner().to_radians();
                let inner = Radians(radians_value);
                Ok(inner)
            }
            _ => Err("Data is not in degrees"),
        };
    }
}
