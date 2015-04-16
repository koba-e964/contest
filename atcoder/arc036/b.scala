object Main extends scala.App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._
  val src = scala.io.Source.fromInputStream(System.in)
  val sc = new ju.Scanner(src.mkString)
  val n = sc.nextInt
  val h = Array.fill(n)(sc.nextInt)
  val asc = Array.fill(n)(0)
  val dsc = Array.fill(n)(0)
  asc(0) = 0
  for (i <- 1 until n) {
    if (h(i - 1) < h(i)) {
      asc(i) = asc(i - 1) + 1
    } else {
      asc(i) = 0
    }
  }
  dsc(n - 1) = 0
  for (i <- n - 2 to 0 by -1) {
    if (h(i) > h(i + 1)) {
      dsc(i) = dsc(i + 1) + 1
    } else {
      dsc(i) = 0
    }
  }
  val ans = (0 until n).map(x => asc(x) + dsc(x) + 1).reduce(_ max _)
  println(ans)
}
