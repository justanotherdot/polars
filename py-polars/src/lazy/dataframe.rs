use crate::dataframe::PyDataFrame;
use crate::error::PyPolarsEr;
use crate::lazy::utils::py_exprs_to_exprs;
use crate::PyExpr;
use polars::lazy::frame::{LazyFrame, LazyGroupBy};
use pyo3::prelude::*;

#[pyclass]
#[repr(transparent)]
pub struct PyLazyGroupBy {
    // option because we cannot get a self by value in pyo3
    pub lgb: Option<LazyGroupBy>,
}

#[pymethods]
impl PyLazyGroupBy {
    pub fn agg(&mut self, aggs: Vec<PyExpr>) -> PyLazyFrame {
        let lgb = self.lgb.take().unwrap();
        let aggs = py_exprs_to_exprs(aggs);
        lgb.agg(aggs).into()
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyLazyFrame {
    // option because we cannot get a self by value in pyo3
    pub ldf: Option<LazyFrame>,
}

impl From<LazyFrame> for PyLazyFrame {
    fn from(ldf: LazyFrame) -> Self {
        PyLazyFrame { ldf: Some(ldf) }
    }
}

#[pymethods]
impl PyLazyFrame {
    pub fn describe_plan(&self) -> String {
        self.ldf.as_ref().unwrap().describe_plan()
    }

    pub fn describe_optimized_plan(&self) -> PyResult<String> {
        let result = self
            .ldf
            .as_ref()
            .unwrap()
            .describe_optimized_plan()
            .map_err(PyPolarsEr::from)?;
        Ok(result)
    }

    pub fn optimization_toggle(
        &mut self,
        type_coercion: bool,
        predicate_pushdown: bool,
        projection_pushdown: bool,
    ) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        let ldf = ldf
            .with_type_coercion_optimization(type_coercion)
            .with_predicate_pushdown_optimization(predicate_pushdown)
            .with_projection_pushdown_optimization(projection_pushdown);
        ldf.into()
    }

    pub fn sort(&mut self, by_column: &str, reverse: bool) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        ldf.sort(by_column, reverse).into()
    }

    pub fn collect(&mut self) -> PyResult<PyDataFrame> {
        let ldf = self.ldf.take().unwrap();
        let df = ldf.collect().map_err(PyPolarsEr::from)?;
        Ok(df.into())
    }

    pub fn filter(&mut self, predicate: PyExpr) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        ldf.filter(predicate.inner).into()
    }

    pub fn select(&mut self, exprs: Vec<PyExpr>) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        let exprs = py_exprs_to_exprs(exprs);
        ldf.select(exprs).into()
    }

    pub fn groupby(&mut self, by: Vec<&str>) -> PyLazyGroupBy {
        let ldf = self.ldf.take().unwrap();
        let lazy_gb = ldf.groupby(by);

        PyLazyGroupBy { lgb: Some(lazy_gb) }
    }

    pub fn inner_join(
        &mut self,
        mut other: PyLazyFrame,
        left_on: PyExpr,
        right_on: PyExpr,
    ) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        let other = other.ldf.take().unwrap();
        ldf.inner_join(other, left_on.inner, right_on.inner).into()
    }

    pub fn outer_join(
        &mut self,
        mut other: PyLazyFrame,
        left_on: PyExpr,
        right_on: PyExpr,
    ) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        let other = other.ldf.take().unwrap();
        ldf.outer_join(other, left_on.inner, right_on.inner).into()
    }

    pub fn left_join(
        &mut self,
        mut other: PyLazyFrame,
        left_on: PyExpr,
        right_on: PyExpr,
    ) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        let other = other.ldf.take().unwrap();
        ldf.left_join(other, left_on.inner, right_on.inner).into()
    }

    pub fn with_column(&mut self, expr: PyExpr) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        ldf.with_column(expr.inner).into()
    }

    pub fn with_columns(&mut self, exprs: Vec<PyExpr>) -> PyLazyFrame {
        let ldf = self.ldf.take().unwrap();
        ldf.with_columns(py_exprs_to_exprs(exprs)).into()
    }

    pub fn clone(&self) -> PyLazyFrame {
        self.ldf.as_ref().unwrap().clone().into()
    }
}
