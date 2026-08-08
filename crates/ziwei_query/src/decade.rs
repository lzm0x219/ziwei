//! 大限 scope 与限内年份定位结果。

use ziwei_core::{Decade, DecadeYear, Natal};

use crate::{DecadeYearOrdinal, Query, scope::Scope};

use crate::scope::impl_scope_queries;

/// 以一个大限命宫为命位建立的 L2 查询 scope。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeScope<'a> {
    fact: &'a Decade,
    pub(crate) scope: Scope<'a>,
}

impl<'a> DecadeScope<'a> {
    pub(crate) fn new(query: Query<'a>, fact: &'a Decade) -> Self {
        Self::from_fact(query.fact(), fact)
    }

    fn from_fact(natal: &'a Natal, fact: &'a Decade) -> Self {
        Self {
            fact,
            scope: Scope::new(natal, fact.ming_palace_branch()),
        }
    }

    /// 返回底层不可变大限事实。
    pub fn fact(self) -> &'a Decade {
        self.fact
    }

    /// 按本限一基序号选择具体年份。
    pub fn year(self, ordinal: DecadeYearOrdinal) -> DecadeYearSelection<'a> {
        DecadeYearSelection::new(self, ordinal)
    }

    /// 返回上一大限；第一大限返回 `None`。
    pub fn previous_decade(self) -> Option<DecadeScope<'a>> {
        let previous = usize::from(self.fact.index().get()).checked_sub(1)?;
        let natal = self.scope.natal();
        natal
            .decades()
            .get(previous)
            .map(|fact| Self::from_fact(natal, fact))
    }

    /// 返回下一大限；最后一限返回 `None`。
    pub fn next_decade(self) -> Option<DecadeScope<'a>> {
        let next = usize::from(self.fact.index().get()) + 1;
        let natal = self.scope.natal();
        natal
            .decades()
            .get(next)
            .map(|fact| Self::from_fact(natal, fact))
    }
}

impl_scope_queries!(DecadeScope);

/// 被定位的大限年份、限内序号及所属大限。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeYearSelection<'a> {
    decade: DecadeScope<'a>,
    ordinal: DecadeYearOrdinal,
}

impl<'a> DecadeYearSelection<'a> {
    pub(crate) const fn new(decade: DecadeScope<'a>, ordinal: DecadeYearOrdinal) -> Self {
        Self { decade, ordinal }
    }

    /// 返回底层不可变大限年份事实。
    pub fn fact(self) -> &'a DecadeYear {
        &self.decade.fact.years()[usize::from(self.ordinal.get() - 1)]
    }

    /// 返回该年份在所属大限中的一基序号。
    pub fn ordinal(self) -> DecadeYearOrdinal {
        self.ordinal
    }

    /// 返回该年份所属的大限 scope。
    pub fn decade(self) -> DecadeScope<'a> {
        self.decade
    }

    /// 返回十二个大限中的上一年，并可跨越大限边界。
    pub fn previous_year(self) -> Option<DecadeYearSelection<'a>> {
        let previous = self.global_index().checked_sub(1)?;
        Some(Self::at_global_index(self.decade.scope.natal(), previous))
    }

    /// 返回十二个大限中的下一年，并可跨越大限边界。
    pub fn next_year(self) -> Option<DecadeYearSelection<'a>> {
        let next = self.global_index() + 1;
        (next < 120).then(|| Self::at_global_index(self.decade.scope.natal(), next))
    }

    fn global_index(self) -> usize {
        usize::from(self.decade.fact.index().get()) * 10 + usize::from(self.ordinal.get() - 1)
    }

    fn at_global_index(natal: &'a Natal, index: usize) -> Self {
        debug_assert!(index < 120);
        let decade = DecadeScope::from_fact(natal, &natal.decades()[index / 10]);
        decade.year(DecadeYearOrdinal::from_zero_based(index % 10))
    }
}
