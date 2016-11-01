#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
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
  int q1 = acc[x + 6 * r][y + 6 * r] - acc[x + r][y + 6 * r] - acc[x + 6 * r][y + r] + acc[x + r][y + r];
  int q2 = acc[nx][ny] - acc[x][ny] - acc[nx][y] + acc[x][y];
  if (q1 != q2) {
    return 0;
  }
  return q1;
}

int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  hash_init();
  ll hsh_a = 12, hsh_b = 16, hsh_c = 11;
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
  cout << va.size() << " " << vb.size() << " " << vc.size() << endl;
}
