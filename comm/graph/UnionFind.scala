/**
 * Union-Find tree.
 * toArray returns an array consisting of the roots of all vertices.
 * Verified by: AtCoder ARC032-B(http://arc032.contest.atcoder.jp/submissions/603918)
 */
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
  def toArray: Array[Int] = {
    for (i <- 0 until n) root(i)
    disj.toArray
  }
}
