/**
 * Dinic's algorithm for maximum flow problem.
 * Verified by: ABC010-D(http://abc010.contest.atcoder.jp/submissions/602810)
 */
final class Dinic(n: Int) {
	import scala.util.control.Breaks.{break, breakable}
	import scala.collection.{mutable => mu}
	import java.util.Arrays
	/* to, cap, rev */
	private[this] val graph = Array.fill(n)(mu.ArrayBuffer[(Int, Int, Int)]())
	private[this] val level = Array.fill(n)(-1)
	private[this] val iter = Array.fill(n)(0)
    /* Perform bfs and calculate distance from s */
  private[this] def bfs(s: Int) {
    Arrays.fill(level, -1)
    val que = mu.Queue(s)
    level(s) = 0
    while (que.nonEmpty) {
      val v = que.dequeue
      for ((eto, ecap, erev) <- graph(v)) {
        if (ecap > 0 && level(eto) == -1) {
          que += eto
          level(eto) = level(v) + 1
        }
      }
    }
  }
  private[this] def dfs(v: Int, t: Int, f: Int): Int = {
    if (v == t) f
    else {
      var ret = 0
      breakable{
        while (iter(v) < graph(v).size) {
          val i = iter(v)
          val (eto, ecap, erev) = graph(v)(i)
          if (ecap > 0 && level(v) < level(eto)) {
            val newf = if (f == -1) ecap else f min ecap
            val d = dfs(eto, t, newf)
            if (d > 0) {
              graph(v)(i) = (eto, ecap - d, erev)
              graph(eto)(erev) = graph(eto)(erev).copy(_2 = graph(eto)(erev)._2 + d)
              ret = d
              break
            }
          }
          iter(v) += 1
        }
      }
      ret
    }
  }
	def addEdge(from: Int, to: Int, cap: Int) {
  	graph(from) += ((to, cap, graph(to).size))
		graph(to) += ((from, 0, graph(from).size - 1))
	}
	def maxFlow(s: Int, t: Int): Int = {
		var flow = 0
		breakable {
			while (true) {
				bfs(s)
				if (level(t) < 0) break
  			java.util.Arrays.fill(iter, 0)
				var f = 0
				breakable {
					while (true) {
						val f = dfs(s, t, -1)
						if (f <= 0) break
						flow += f;
					}
				}
			}
		}
		flow
	}
}


/*
 * Reference: http://lizan.asia/blog/2012/12/11/scala-competitive/
 */

object Main extends App {
	import java.{util => ju}
	import scala.annotation._
	import scala.collection._
	import scala.collection.{mutable => mu}
	import scala.collection.JavaConverters._
	import scala.math._

	val sc = new ju.Scanner(System.in)
  val n, g, e = sc.nextInt
  val din = new Dinic(n + 1)
  for (_ <- 0 until g) {
    din.addEdge(sc.nextInt, n, 1)
  }
	for (_ <- 0 until e) {
	  val x, y = sc.nextInt
	  din.addEdge(x, y, 1)
	  din.addEdge(y, x, 1)
	}
	println(din.maxFlow(0, n))
}