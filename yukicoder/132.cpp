#include <iostream>
#include <vector>
#include <cstdio>
#include <cmath>
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
const double EPS=1e-9;

const int N = 310;
double p[3];
double q[N][3];

double solve(int i, int j, int k) {
  double op[3];
  double op2 = 0;
  REP (a, 0, 3) {
    int b = (a + 1) % 3;
    int c = (a + 2) % 3;
    op[a] = (q[j][b] - q[i][b]) * (q[k][c] - q[i][c]) 
      - (q[j][c] - q[i][c]) * (q[k][b] - q[i][b]);
    op2 += op[a] * op[a];
  }
  double s = 0;
  REP(a,0,3) {
    s += (p[a] - q[i][a]) * op[a];
  }
  return abs(s) / sqrt(op2);
}

int main(void){
  int n;
  cin >> n >> p[0] >> p[1] >> p[2];
  REP (i, 0, n) {
    cin >> q[i][0] >> q[i][1] >> q[i][2];
  }
  double sum = 0;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      REP(k, j + 1, n) {
	sum += solve(i,j,k);
      }
    }
  }
  printf("%.10f\n", sum);

}
