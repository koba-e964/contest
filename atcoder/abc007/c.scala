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
  val r,c = sc.nextInt
  val sy,sx,gy,gx = sc.nextInt - 1
  val board = Array.fill(r)(sc.next)
  val inf = 100000000
  val dp = Array.fill(r)(Array.fill(c)(inf))
  val que = mu.Queue((0, sy, sx))
  val dx = Array(1, 0, -1, 0)
  val dy = Array(0, 1, 0, -1)
  while (que.nonEmpty) {
    val (d, y, x) = que.dequeue
    if (board(y)(x) == '.') {
      if (d < dp(y)(x)) {
        dp(y)(x) = d
        que ++= (0 until 4).map{i => (d + 1, y + dy(i), x + dx(i))}
      }
    }
  }
  println(dp(gy)(gx))
}