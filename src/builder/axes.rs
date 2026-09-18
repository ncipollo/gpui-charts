//! Builder for [`Axes`].

use crate::axes::Axes;
use crate::axes::horizontal::HorizontalAxis;
use crate::axes::label::AxisLabel;
use crate::axes::style::AxisStyle;
use crate::axes::vertical::VerticalAxis;
use crate::error::ChartError;

/// Builds an [`Axes`] value from label series at normalized positions.
///
/// An axis is only created when labels or a style have been supplied for it.
#[derive(Debug, Default)]
pub struct AxesBuilder {
    horizontal: Option<Vec<AxisLabel>>,
    vertical: Option<Vec<AxisLabel>>,
    horizontal_style: Option<AxisStyle>,
    vertical_style: Option<AxisStyle>,
}

impl AxesBuilder {
    /// Creates an empty axes builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the horizontal axis labels, creating the axis if needed.
    pub fn horizontal_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<AxisLabel>>,
    ) -> Self {
        self.horizontal
            .get_or_insert_with(Vec::new)
            .extend(labels.into_iter().map(Into::into));
        self
    }

    /// Sets the vertical axis labels, creating the axis if needed.
    pub fn vertical_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<AxisLabel>>,
    ) -> Self {
        self.vertical
            .get_or_insert_with(Vec::new)
            .extend(labels.into_iter().map(Into::into));
        self
    }

    /// Sets the horizontal axis style, creating the axis if needed.
    pub fn horizontal_style(mut self, style: AxisStyle) -> Self {
        self.horizontal_style = Some(style);
        self.horizontal.get_or_insert_with(Vec::new);
        self
    }

    /// Sets the vertical axis style, creating the axis if needed.
    pub fn vertical_style(mut self, style: AxisStyle) -> Self {
        self.vertical_style = Some(style);
        self.vertical.get_or_insert_with(Vec::new);
        self
    }

    /// Validates label positions and builds the axes.
    pub fn build(self) -> Result<Axes, ChartError> {
        let horizontal = self
            .horizontal
            .map(|labels| {
                validate_labels(&labels)?;
                let axis = HorizontalAxis::new(labels);
                Ok(match self.horizontal_style {
                    Some(style) => axis.with_style(style),
                    None => axis,
                })
            })
            .transpose()?;
        let vertical = self
            .vertical
            .map(|labels| {
                validate_labels(&labels)?;
                let axis = VerticalAxis::new(labels);
                Ok(match self.vertical_style {
                    Some(style) => axis.with_style(style),
                    None => axis,
                })
            })
            .transpose()?;
        Ok(Axes::new(horizontal, vertical))
    }
}

fn validate_labels(labels: &[AxisLabel]) -> Result<(), ChartError> {
    match labels
        .iter()
        .enumerate()
        .find(|(_, l)| !(0.0..=1.0).contains(&l.position))
    {
        Some((index, label)) => Err(ChartError::LabelOutOfRange {
            index,
            position: label.position,
        }),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::AxesBuilder;
    use crate::error::ChartError;

    #[test]
    fn empty_builder_has_no_axes() {
        let axes = AxesBuilder::new().build().expect("valid");
        assert!(axes.horizontal().is_none());
        assert!(axes.vertical().is_none());
    }

    #[test]
    fn labels_create_axes() {
        let axes = AxesBuilder::new()
            .horizontal_labels([(0.0, "a"), (1.0, "b")])
            .vertical_labels([(0.5, "mid")])
            .build()
            .expect("valid");
        assert_eq!(axes.horizontal().map(|a| a.labels().len()), Some(2));
        assert_eq!(axes.vertical().map(|a| a.labels().len()), Some(1));
    }

    #[test]
    fn rejects_out_of_range_label() {
        let err = AxesBuilder::new()
            .horizontal_labels([(0.0, "a"), (1.2, "b")])
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ChartError::LabelOutOfRange {
                index: 1,
                position: 1.2
            }
        );
    }
}
