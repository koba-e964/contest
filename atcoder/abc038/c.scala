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
val a = Array.fill(n)(sc.nextLong)
var sum = 0L
var cnt = 0L
for (i <- 0 until n) {
  if (i == 0 || a(i - 1) < a(i)) {
    cnt += 1L
  } else {
 cnt = 1L
  }
  sum += cnt
  
}
println(sum)
}