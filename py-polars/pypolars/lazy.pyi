from typing import Union

class Expr:
    def eq(self, other: Expr) -> Expr: ...
    def neq(self, other: Expr) -> Expr: ...
    def gt(self, other: Expr) -> Expr: ...
    def gt_eq(self, other: Expr) -> Expr: ...
    def lt_eq(self, other: Expr) -> Expr: ...
    def lt(self, other: Expr) -> Expr: ...
    def alias(self, name: str) -> Expr: ...
    def is_not(self) -> Expr: ...
    def is_null(self) -> Expr: ...
    def is_not_null(self) -> Expr: ...
    def agg_min(self) -> Expr: ...
    def agg_max(self) -> Expr: ...
    def agg_mean(self) -> Expr: ...
    def agg_median(self) -> Expr: ...
    def agg_sum(self) -> Expr: ...
    def agg_n_unique(self) -> Expr: ...
    def agg_first(self) -> Expr: ...
    def agg_last(self) -> Expr: ...
    def agg_quantile(self, quantile: float) -> Expr: ...
    def agg_groups(self) -> Expr: ...
    def cast(self, data_type: str) -> Expr: ...
    def sort(self, reverse: bool) -> Expr: ...
    def shift(self, periods: int) -> Expr: ...

class WhenThen:
    def otherwise(self, expr: Expr) -> Expr: ...

class When:
    def then(self, expr: Expr) -> WhenThen: ...

def col(name: str) -> Expr: ...
def lit(value: Union[float, int]) -> Expr: ...
def when(expr: Expr) -> WhenThen: ...
