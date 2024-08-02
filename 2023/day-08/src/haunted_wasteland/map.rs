use std::{error::Error, str::FromStr};

use common::parse_struct_error::ParseStructError;
use num::Integer;

use super::directions::{Direction, Directions};

#[derive(Debug, Clone)]
pub struct Map {
	nodes: Vec<MapNode>,
}

#[derive(Debug, Clone)]
pub struct MapNode {
	id: String,
	left: usize,
	right: usize,
}

#[derive(Debug, Clone)]
pub struct MapBuilder {
	nodes: Vec<MapBuilderNode>,
}

#[derive(Debug, Clone)]
pub struct MapBuilderNode {
	id: String,
	left: String,
	right: String,
}

impl Map {
	pub fn traverse(
		&self,
		directions: &Directions,
		is_start: fn(&str) -> bool,
		is_end: fn(&str) -> bool,
	) -> usize {
		self.nodes
			.iter()
			.enumerate()
			.filter_map(|(i, e)| {
				if is_start(&e.id) {
					Some(self.traverse_one(directions, i, is_end))
				} else {
					None
				}
			})
			.fold(1, |total, n| total.lcm(&n))
	}

	fn traverse_one(
		&self,
		directions: &Directions,
		start: usize,
		is_end: fn(&str) -> bool,
	) -> usize {
		directions
			.iter()
			.enumerate()
			.scan(start, |head, (i, dir)| {
				if !is_end(&self.nodes[*head].id) || i % directions.len() != 0 {
					*head = match *dir {
						Direction::Left => self.nodes[*head].left,
						Direction::Right => self.nodes[*head].right,
					};
					Some(())
				} else {
					None
				}
			})
			.count()
	}
}

impl MapBuilder {
	pub fn new() -> Self {
		Self { nodes: vec![] }
	}

	pub fn add_node(mut self, node: MapBuilderNode) -> Self {
		self.nodes.push(node);
		self
	}

	pub fn build(self) -> Result<Map, Box<dyn Error>> {
		let nodes = self
			.nodes
			.iter()
			.map(|a| {
				Ok::<MapNode, Box<dyn Error>>(MapNode {
					id: a.id.clone(),
					left: self
						.nodes
						.iter()
						.enumerate()
						.find_map(|(i, b)| if a.left == b.id { Some(i) } else { None })
						.ok_or(format!("invalid left mapping {}", a.left))?,
					right: self
						.nodes
						.iter()
						.enumerate()
						.find_map(|(i, b)| if a.right == b.id { Some(i) } else { None })
						.ok_or(format!("invalid right mapping  {}", a.right))?,
				})
			})
			.collect::<Result<Vec<_>, _>>()?;
		// let start = self
		// 	.nodes
		// 	.iter()
		// 	.enumerate()
		// 	.find(|(_, e)| e.id == "AAA")
		// 	.map(|(i, _)| i)
		// 	.ok_or("start node not found")?;
		// let end = self
		// 	.nodes
		// 	.iter()
		// 	.enumerate()
		// 	.find(|(_, e)| e.id == "ZZZ")
		// 	.map(|(i, _)| i)
		// 	.ok_or("end node not found")?;

		Ok(Map { nodes })
	}
}

impl FromStr for MapBuilderNode {
	type Err = ParseStructError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (id, routes) = s
			.split_once(" = ")
			.ok_or(ParseStructError::new::<Self>(s))?;
		let (left, right) = routes
			.trim_matches(['(', ')'])
			.split_once(", ")
			.ok_or(ParseStructError::new::<Self>(s))?;

		Ok(MapBuilderNode {
			id: id.to_string(),
			left: left.to_string(),
			right: right.to_string(),
		})
	}
}
