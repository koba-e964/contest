#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 3;
const double EPS = 1e-9;
double x[N],y[N],r[N];

double squ(double x) {
  return x * x;
}


bool disj(int a, int b) {
  double sq = squ(x[a] - x[b]) + squ(y[a] - y[b]);
  double lim = r[a] + r[b];
  return sq > lim * lim - EPS;
}
bool contains(int a, int b) {
  double sq = squ(x[a] - x[b]) + squ(y[a] - y[b]);
  double lim = r[a] - r[b];
  return lim > -EPS && sq < lim * lim + EPS;
}

bool in(int a, double xx, double yy) {
  double sq = squ(xx - x[a]) + squ(yy - y[a]);
  return sq <= r[a] * r[a] - EPS;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> x[i] >> y[i] >> r[i];
  }
  VI poss(1 << n, 1);
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (i == j) continue;
      if (disj(i, j)) {
	int b = 1 << i | 1 << j;
	REP(bits, 0, 1 << n) {
	  if ((b & bits) == b) {
	    poss[bits] = 0;
	  }
	}
      }
      if (contains(i, j)) {
	int b = 1 << i | 1 << j;
	int c = 1 << j;
	REP(bits, 0, 1 << n) {
	  if ((b & bits) == c) {
	    poss[bits] = 0;
	  }
	}
      }
    }
  }
  if (n == 3) {
    // do three circles intersect?
    const int div = 3000;
    poss[7] = 0;
    int ins = 0;
    REP(i, 0, div + 1) {
      if (ins)break;
      REP(j, 0, div + 1) {
	double xx = (double) i / div * (2 * r[0]) + x[0] - r[0];
	double yy = (double) j / div * (2 * r[0]) + y[0] - r[0];
	bool ok=1;
	REP(k, 0, 3) {
	  if (not in(k, xx, yy)) {
	    ok = 0;
	    break;
	  }
	}
	if (ok) {
	  //cerr << "(x,y)="<<xx<<" "<<yy<<endl;
	  ins = 1;
	  break;
	}
      }
    }
    poss[7] = ins;
  }
  int c = 0;
  REP(i, 0, 1 << n) {
    c += poss[i];
    //cerr<<poss[i]<<" ";
  }
  cout << c << "\n";
}
