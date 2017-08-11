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
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int query(int si, int sj, int ti, int tj) {
  int ans;
  cout << si + 1 << " " << sj + 1 << " " << ti + 1 << " " << tj + 1 << endl;
  cin >> ans;
  return ans;
}

int main(void) {
  int h, w, q;
  cin >> h >> w >> q;
  vector<VI> row(h, VI(w - 1, 0));
  vector<VI> col(h - 1, VI(w, 0));
  REP(i, 0, h) {
    REP(j, 0, w - 1) {
      int ans = query(i, j, i, j + 1);
      row[i][j] = ans;
    }
  }
  REP(i, 0, h - 1) {
    int ans = query(i, 0, i + 1, 0);
    col[i][0] = ans;
  }
  query(0, 0, 0, 0); // no effect
  REP(i, 0, h - 1) {
    REP(j, 1, w) {
      col[i][j] = col[i][j - 1] + row[i + 1][j - 1] - row[i][j - 1];
    }
  }
  vector<VL> dist(h * w, VL(h * w, 1e15));
  REP(i, 0, h) {
    REP(j, 0, w - 1) {
      dist[i * w + j][i * w + j + 1] = row[i][j];
      dist[i * w + j + 1][i * w + j] = row[i][j];
    }
  }
  REP(i, 0, h * w) {
    dist[i][i] = 0;
  }
  REP(i, 0, h - 1) {
    REP(j, 0, w) {
      dist[i * w + j][i * w + j + w] = col[i][j];
      dist[i * w + j + w][i * w + j] = col[i][j];
    }
  }
  REP(k, 0, h * w) {
    REP(i, 0, h * w) {
      REP(j, 0, h * w) {
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  REP(i, 0, q) {
    int si, sj, ti, tj;
    cin >> si >> sj >> ti >> tj;
    si--, sj--, ti--, tj--;
    cout << dist[si * w + sj][ti * w + tj] << endl;
  }
}
