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
  val n, k = sc.nextInt
  val a = Array.fill(n)(sc.nextDouble).sorted
  var p = 0.5
  var sum = 0.0
  for (i <- 0 until k) {
    sum += a(n - i - 1) * p
    p *= 0.5
  }
  println(sum)
}