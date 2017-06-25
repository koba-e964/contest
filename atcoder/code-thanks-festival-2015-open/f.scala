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
  val n = sc.nextInt
  val edges = Array.fill(n - 1)((sc.nextInt, sc.nextInt))
  val deg1 = edges.count{case (a, b) => a == 1}
  println(if (deg1 == 1) "A" else if (n % 2 == 0) "A" else "B")

}