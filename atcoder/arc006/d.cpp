#include <algorithm>
#include <cassert>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
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

const int H = 1010;
int h, w;
string s[H];
int acc[H][H];
string a[7] = {
  ".......",
  "...o...",
  "..o.o..",
  ".o...o.",
  ".ooooo.",
  ".o...o.",
  "......."
}; // 12

string b[7] = {
  ".......",
  ".oooo..",
  ".o...o.",
  ".oooo..",
  ".o...o.",
  ".oooo..",
  "......."
}; // 16
string c[7] = {
  ".......",
  "..ooo..",
  ".o...o.",
  ".o.....",
  ".o...o.",
  "..ooo..",
  "......."
}; // 11
void hash_init(void) {
  REP(i, 0, h) {
    REP(j, 0, w) {
      acc[i+1][j+1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] + (s[i][j] == 'o');
    }
  }
}

ll hash_func(int x, int y, int r) {
  assert (r >= 1);
  assert (x >= 0);
  assert (x + 7 * r <= h);
  assert (y >= 0);
  assert (y + 7 * r <= w);
  int nx = x + 7 * r;
  int ny = y + 7 * r;
  // check the margin
  int q1 = acc[x + r][ny] - acc[x][ny] - acc[x + r][y] + acc[x][y];
  int q2 = acc[nx][ny] - acc[x + 6 * r][ny] - acc[nx][y] + acc[x + 6 * r][y];
  if (q2 > 0 || q2 > 0) {
    return 0;
  }
  return acc[nx][ny] - acc[x][ny] - acc[nx][y] + acc[x][y];
}

bool check(int x, int y, int r, string tbl[7]) {
  assert (r >= 1);
  assert (x >= 0);
  assert (x + 7 * r <= h);
  assert (y >= 0);
  assert (y + 7 * r <= w);
  bool ok = 1;
  for (int i = 0; i < 7 && ok; ++i) {
    for (int j = 0; j < 7 && ok; ++j) {
      REP(k, 0, r) REP(l, 0, r) {
	if (s[x + i * r + k][y + j * r + l] != tbl[i][j]) {
	  ok = 0;
	  break;
	}
      }
    }
  }
  return ok;
}

int check_pat(const vector<PI> &cand, string pat[7]) {
  string rot[7];
  int cnt = 0;
  REP(loop_cnt, 0, 4) {
    REP(i, 0, 7) {
      rot[i] = pat[i];
    }
    REP(i, 0, cand.size()) {
      int c = cand[i].first;
      int x = c / w, y = c % w;
      int r = cand[i].second;
      if (check(x, y, r, rot)) {
	//cout << "[" << loop_cnt << "]:" << x << " " << y << " " << r << endl;
	cnt++;
      }
    }
    REP(i, 0, 7) {
      REP(j, 0, 7) {
	pat[i][j] = rot[6 - j][i]; // rotate by -90 degree (90 degree clockwise)
      }
    }
  }
  return cnt;
}

int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  hash_init();
  ll hsh_a = 12, hsh_b = 16, hsh_c = 11;
  int cnt_a = 0, cnt_b = 0, cnt_c = 0;
  vector<PI> va, vb, vc;

  REP(r, 1, 150) {
    REP(x, 0, h - 7 * r + 1) {
      REP(y, 0, w - 7 * r + 1) {
	ll hsh = hash_func(x, y, r);
	if (hsh == hsh_a * r * r) {
	  va.push_back(PI(x * w + y, r));
	}
	if (hsh == hsh_b * r * r) {
	  vb.push_back(PI(x * w + y, r));
	}
	if (hsh == hsh_c * r * r) {
	  vc.push_back(PI(x * w + y, r));
	}
      }
    }
  }
  cnt_a = check_pat(va, a);
  cnt_b = check_pat(vb, b);
  cnt_c = check_pat(vc, c);
  cout << cnt_a << " " << cnt_b << " " << cnt_c << endl;
}
