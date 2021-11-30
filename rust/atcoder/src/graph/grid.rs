use super::Graph;
use super::Edge;

#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct Grid<T> {
    x_size: usize,
    y_size: usize,
    graph: Graph<T>,
}

impl <T> Grid<T> {
    pub fn new(x_size: usize, y_size:usize, graph: Graph<T>) -> Self {
        return Self { x_size, y_size, graph };
    }
    pub fn graph(&self) -> &Graph<T> {
        return &self.graph;
    }
    pub fn coord_to_node(&self, x:usize, y:usize) -> usize {
        if x >= self.x_size || y >= self.y_size {
            panic!("x >= x_size: {:?} >= {:?} or y >= y_size: {:?} >= {:?}", x, self.x_size, y, self.y_size);
        }
        let mut offset_base = 1;
        let mut node = offset_base*x;
        offset_base *= self.x_size;
        node += offset_base*y;
        return node;
    }
    pub fn node_to_coord(&self, node:usize) -> (usize, usize) {
        if node >= self.x_size * self.y_size {
            panic!("node >= self.x_size * self.y_size: {:?} >= {:?} * {:?}", node, self.x_size, self.y_size);
        }
        let x= node % self.x_size;
        let y= node / self.x_size;
        return (x,y);
    }
    fn edges_from_node<F>(&mut self, x: usize, y:usize, delta_x: &[i64], delta_y: &[i64], should_skip: F) -> Vec<(usize,usize)>
        where F: Fn(usize,usize) -> bool {
        let mut edges = Vec::with_capacity(delta_x.len());
        for i in 0..delta_x.len() {
            let x2 = x as i64 + delta_x[i];
            let y2 = y as i64 + delta_y[i];
            if x2 < 0 || y2 < 0 {
                continue;
            }
            let x2 = x2 as usize;
            let y2 = y2 as usize;
            if x2 >= self.x_size || y2 >= self.y_size || should_skip(x2,y2) {
                continue;
            }
            let u = self.coord_to_node(x,y);
            let v = self.coord_to_node(x2,y2);
            edges.push((u,v));
        }
        return edges;
    }
}

impl Grid<Edge> {
    pub fn add_edge(&mut self, u:usize,v:usize) {
        self.graph.add_edge(u,v);
    }
    pub fn construct_node<F>(&mut self, x: usize, y:usize, delta_x: &[i64], delta_y: &[i64], should_skip: F)
        where F: Fn(usize,usize) -> bool {
        // let delta_x = [-1,1,0,0];
        // let delta_y = [0,0,-1,1];
        for (u,v) in self.edges_from_node(x,y,delta_x,delta_y,should_skip) {
            self.add_edge(u,v);
        }
    }
    pub fn debug_print(&self) {
        for edge in &self.graph.edges {
            println!("{:?}: {:?} -> {:?}", edge, self.node_to_coord(edge.u), self.node_to_coord(edge.v));
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn grid() {
        let x=3;
        let y=3;
        let dx = [-1,1,0,0];
        let dy = [0,0,-1,1];
        let graph = Graph::<Edge>::new(x*y, (x*y)*dx.len());
        let mut grid = Grid::new(x, y, graph);
        for x1 in 0..x {
            for y1 in 0..y {
                grid.construct_node(x1,y1,&dx,&dy,|_,_| false);
            }
        }
        let expectd = &[
        //  u      , v
            ((0, 0), (1, 0)),
            ((0, 0), (0, 1)),
            ((0, 1), (1, 1)),
            ((0, 1), (0, 0)),
            ((0, 1), (0, 2)),
            ((0, 2), (1, 2)),
            ((0, 2), (0, 1)),
            ((1, 0), (0, 0)),
            ((1, 0), (2, 0)),
            ((1, 0), (1, 1)),
            ((1, 1), (0, 1)),
            ((1, 1), (2, 1)),
            ((1, 1), (1, 0)),
            ((1, 1), (1, 2)),
            ((1, 2), (0, 2)),
            ((1, 2), (2, 2)),
            ((1, 2), (1, 1)),
            ((2, 0), (1, 0)),
            ((2, 0), (2, 1)),
            ((2, 1), (1, 1)),
            ((2, 1), (2, 0)),
            ((2, 1), (2, 2)),
            ((2, 2), (1, 2)),
            ((2, 2), (2, 1)),
        ];
        for i in 0..grid.graph.edges.len() {
            let Edge { u, v } = grid.graph.edges[i];
            assert_eq!(expectd[i].0, grid.node_to_coord(u));
            assert_eq!(expectd[i].1, grid.node_to_coord(v));
        }
    }
}
