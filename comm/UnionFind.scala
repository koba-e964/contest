class UnionFind(n: Int) {
  private[this] val disj = Array.tabulate(n)(x => x)
  def root(x: Int): Int =
    if (disj(x) != x) {
      disj(x) = root(disj(x))
      disj(x)
    } else x
  def unite(x: Int, y: Int) {
    disj(root(y)) = root(x)
  }
  def toArray: Array[Int] = disj.to
}
