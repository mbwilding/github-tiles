use log::debug;
use oxvg_ast::parse::roxmltree::parse;
use oxvg_ast::serialize::Node as _;
use oxvg_ast::visitor::Info;
use oxvg_optimiser::Jobs;

/// Optimizes an SVG string
pub fn optimize(svg: &str) -> String {
    debug!("Optimizing SVG ({} bytes)", svg.len());
    let result = parse(svg, |dom, allocator| {
        let jobs = Jobs::default();
        if jobs.run(dom, &Info::new(allocator)).is_ok() {
            dom.serialize().unwrap_or_else(|_| svg.to_string())
        } else {
            svg.to_string()
        }
    })
    .unwrap_or_else(|_| svg.to_string());
    debug!("SVG optimized ({} bytes)", result.len());
    result
}
