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

const int N = 310;

int fst[N][N];
int snd[N][N];

int ask_lt(int x, int y) {
  assert (x != y);
  cout << "? " << x << " " << y << endl;
  int ans;
  cin >> ans;
  return ans;
}

int calc(int x, int y, int rank) {
  assert (x <= y);
  assert (rank == 1 || rank == 2);
  if (rank == 2 && snd[x][y] > 0) {
    return snd[x][y];
  }
  if (rank == 1) {
    if (fst[x][y] > 0) {
      return fst[x][y];
    }
    if (y == x) {
      return fst[x][y] = x;
    }
    if (y == x + 1) {
      int m = ask_lt(x, y);
      return fst[x][y] = m;
    }
    int mid = (x + y) / 2;
    int former = calc(x, mid, 1);
    int latter = calc(mid + 1, y, 1);
    int m = ask_lt(former, latter);
    return fst[x][y] = m;
  }
  // rank == 2
  if (snd[x][y] > 0) {
    return snd[x][y];
  }
  assert (x < y);
  assert (fst[x][y] > 0);
  if (y == x + 1) {
    int m = fst[x][y];
    return snd[x][y] = x + y - m;
  }
  int mid = (x + y) / 2;
  int former = calc(x, mid, 1);
  int latter = calc(mid + 1, y, 1);
  int m = fst[x][y];
  int ans;
  if (m == former) {
    int fs = calc(x, mid, 2);
    int m2 = ask_lt(fs, latter);
    ans = m2;
  } else {
    if (y > mid + 1) {
      int ls = calc(mid + 1, y, 2);
      int m2 = ask_lt(ls, former);
      ans = m2;
    } else {
      ans = former;
    }
  }
  return snd[x][y] = ans;
}


int main(void) {
  int n;
  cin >> n;
  REP(i, 0, N) {
    REP(j, 0, N) {
      fst[i][j] = -1;
      snd[i][j] = -1;
    }
  }
  calc(1, n, 1);
  int ans = calc(1, n, 2);
  cout << "! " << ans << endl;
}
