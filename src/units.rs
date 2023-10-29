pub trait Inner {
    type InnerType;
    fn inner(&self) -> Self::InnerType;
}

#[derive(Debug)]
pub enum LengthUnit {
    Km,
    M,
}

#[derive(Debug)]
pub struct Degrees(pub f64);

#[derive(Debug)]
pub struct Radians(pub f64);

#[derive(Debug)]
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
