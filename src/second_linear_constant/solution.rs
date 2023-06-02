use crate::second_linear_constant::initial_value as iv;
use crate::second_linear_constant::boundary_value as bv;
use crate::math;

#[derive(Debug)]
pub enum Solution
{
    Double { r1: f64, r2: f64, alpha: f64, beta: f64 }, // alpha*(r1*x).exp()+beta*(r2*x).exp()
    Simple { r0: f64, alpha: f64, beta: f64 }, // (alpha*x + beta)*(r0*x).exp()
    Complex { p: f64, q: f64, alpha: f64, beta: f64 }, // (p*x).exp()*(alpha*(q*x).cos() + beta*(q*x).sin()) where r1 = p + iq and r2 = p - iq
}

impl Solution
{
    pub fn y(&self, x: f64) -> f64
    {
        match self
        {
            Solution::Double { r1, r2, alpha, beta, } => alpha*(r1*x).exp()+beta*(r2*x).exp(),
            Solution::Simple { r0, alpha, beta } => (alpha*x + beta)*(r0*x).exp(), 
            Solution::Complex { p, q, alpha, beta } => (p*x).exp()*(alpha*(q*x).cos() + beta*(q*x).sin()),
        }
    }
}

impl From<&iv::Problem> for Solution
{
    fn from(problem: &iv::Problem) -> Self
    {
        let iv::Problem { a, b, y0_prime: y0p, .. } = problem;
        let y0 = problem.y0;
        
        let delta_r = a.powi(2) - 4.0*b;
        if math::is_approx_zero(delta_r)
        {
            let r0 = -a/2.0;
            let alpha = y0p - y0*r0;
            return Self::Simple { r0, alpha, beta: y0 };
        }
        if delta_r > 0.0
        {
            let r1 = (-a + delta_r.sqrt())/2.0;
            let r2 = (-a - delta_r.sqrt())/2.0;
            
            let alpha = (y0p - r2*y0)/(r1 - r2);
            let beta = (y0*r1 - y0p)/(r1 - r2);
            return Self::Double { r1, r2, alpha, beta };
        }
        else
        {
            let p = -a/2.0;
            let q = delta_r.abs().sqrt()/2.0;
            
            let beta = (y0p - y0*p)/q;
            
            return Self::Complex { p, q, alpha: y0, beta };
        }
    }
}

impl From<&bv::Problem> for Solution
{
    fn from(problem: &bv::Problem) -> Self
    {
        let solutions = Solutions::from(problem);
        
        let a = problem.a;
        let b = problem.b;
        let y0 = problem.y0;
        let y1 = problem.y1;
        
        let c = solutions.c;
        
        // Find the root of F(a) = solutions.y1(a) - problem.y1
        let root = (y1 - c.1)/c.0;
        let iv_problem = iv::Problem{ a, b, y0, y0_prime: root };
        
        Self::from(&iv_problem)
    }
}

#[derive(Debug)]
pub struct Solutions
{
    pub c: (f64, f64),
}

impl Solutions
{
    pub fn y1(&self, a: f64) -> f64
    {
        a*self.c.0 + self.c.1
    }
}

impl From<&bv::Problem> for Solutions
{
    fn from(problem: &bv::Problem) -> Self
    {
        let bv::Problem { a, b, y0, x1, .. } = problem;
        
        let delta_r = a.powi(2) - 4.0*b;
        if math::is_approx_zero(delta_r)
        {
            let r0 = -a/2.0;
            
            let c1 = x1*(r0*x1).exp();
            let c2 = y0*(r0*x1).exp()*(1.0 - x1*r0);
            
            return Self { c: (c1, c2) };
        }
        if delta_r > 0.0
        {
            let r1 = (-a + delta_r.sqrt())/2.0;
            let r2 = (-a - delta_r.sqrt())/2.0;
            
            let c1 = ((r1*x1).exp() - (r2*x1).exp())/(r1 - r2);
            let c2 = y0/(r1 - r2)*(r1*(r2*x1).exp() - r2*(r1*x1).exp());
            return Self { c: (c1, c2) };
        }
        else
        {
            let p = -a/2.0;
            let q = delta_r.abs().sqrt()/2.0;
            
            let c1 = (q*x1).sin()*(p*x1).exp()/q;
            let c2 = y0*(p*x1).exp()*((q*x1).cos() - (q*x1).sin()*p/q);
            
            return Self { c: (c1, c2) };
        }
    }
}

