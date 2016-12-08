#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 1025;
int bit[N][N];

void add(int x, int y) {
  assert (x > 0);
  assert (y > 0);
  while (x < N) {
    int idy = y;
    while (idy < N) {
      bit[x][idy] ^= 1;
      idy += idy & (-idy);
    }
    x += x & (-x);
  }
}

int acc(int x, int y) {
  int sum = 0;
  while (x > 0) {
    int idy = y;
    while (idy > 0) {
      sum ^= bit[x][idy];
      idy &= idy - 1;
    }
    x &= x - 1;
  }
  return sum;
}


int main(void){
  int x;
  scanf("%d", &x);
  while(x--) {
    REP(i, 0, N) {
      REP(j, 0, N) {
	bit[i][j] = 0;
      }
    }
    int n, t;
    scanf("%d%d", &n, &t);
    REP(i, 0, t) {
      char c[2];
      int x, y;
      scanf("%1s%d%d", c, &x, &y);
      if (c[0] == 'C') {
	int z, w;
	scanf("%d%d", &z, &w);
	add(x, y);
	add(x, w + 1);
	add(z + 1, y);
	add(z + 1, w + 1);
	if (0) {
	  REP(i, 1, 10) {
	    cerr << "[";
	    REP(j, 1, 10) {
	      cerr << acc(i, j) << " ";
	    }
	    cerr << "]" << endl;
	  }
	}
      } else if (c[0] == 'Q') {
	printf("%d\n", acc(x, y));
      } else {
	assert (0);
      }
    }
    if (x > 0) { puts(""); }
  }
}
