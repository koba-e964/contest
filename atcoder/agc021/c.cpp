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
int n, m, a, b;

void fail(void) {
  cout << "NO\n";
  exit(0);
}

vector<string> trans(const vector<string> &ans) {
  if (ans.size() == 0) return ans;
  int n = ans.size();
  int m = ans[0].size();
  vector<string> ret(m, string(n, '.'));
  map<char, char> tbl;
  tbl['.'] = '.';
  tbl['<'] = '^';
  tbl['>'] = 'v';
  tbl['^'] = '<';
  tbl['v'] = '>';
  REP(i, 0, n) {
    REP(j, 0, m) {
      ret[j][i] = tbl[ans[i][j]];
    }
  }
  return ret;
}


bool is_ok_even_even(int n, int m, int a, int b) {
  a = (a + 1) / 2 * 2;
  b = (b + 1) / 2 * 2;
  return n * m >= 2 * a + 2 * b;
}


vector<string> even_even(int n, int m, int a, int b) {
  assert (n % 2 == 0);
  assert (m % 2 == 0);
  if (not is_ok_even_even(n, m, a, b)) return vector<string>();
  vector<string> ret(n, string(m, '.'));
  REP(i, 0, n / 2) {
    REP(j, 0, m / 2) {
      if (a > 0) {
	ret[2 * i][2 * j] = '<';
	ret[2 * i][2 * j + 1] = '>';
	a--;
	if (a >= 1) {
	  ret[2 * i + 1][2 * j] = '<';
	  ret[2 * i + 1][2 * j + 1] = '>';
	  a--;
	}
      } else if (b > 0) {
	ret[2 * i][2 * j] = '^';
	ret[2 * i + 1][2 * j] = 'v';
	b--;
	if (b >= 1) {
	  ret[2 * i][2 * j + 1] = '^';
	  ret[2 * i + 1][2 * j + 1] = 'v';
	  b--;
	}
      }
    }
  }
  return ret;
}

vector<string> even_odd(int n, int m, int a, int b) {
  assert (n % 2 == 0);
  assert (m % 2 == 1);
  assert (n * m >= 2 * a + 2 * b);
  vector<string> ret(n, string(m, '.'));
  int edge_b = min(b, n / 2);
  int vero_b = b - edge_b;
  int edge_a = 0;
  int vero_a = a - edge_a;
  if (not is_ok_even_even(n, m - 1, vero_a, vero_b)) {
    return vector<string>();
  }
  vector<string> vero = even_even(n, m - 1, vero_a, vero_b);
  REP(i, 0, n) {
    REP(j, 0, m - 1) {
      ret[i][j] = vero[i][j];
    }
  }
  if (m % 2 == 1) {
    REP(i, 0, edge_b) {
      ret[2 * i][m / 2 * 2] = '^';
      ret[2 * i + 1][m / 2 * 2] = 'v';
    }
  }
  return ret;
}

vector<string> with3x3(int n, int m, int a, int b) {
  assert (n % 2 == 1 && n > 1);
  assert (m % 2 == 1 && m > 1);
  int edge_a = min(a, m / 2 + 1);
  int edge_b = min(b, n / 2 + 1);
  int vero_a = a - edge_a;
  int vero_b = b - edge_b;
  if (not is_ok_even_even(n - 1, m - 1, vero_a, vero_b + 2)) {
    return vector<string>();
  }
  vector<string> inn = even_even(n - 1, m - 1, vero_a, vero_b);
  vector<string> ret(n, string(m, '.'));
  REP(i, 0, n - 1) {
    REP(j, 0, m - 1) {
      ret[i][j] = inn[i][j];
    }
  }
  // Trick. This cells can't be <>^v because of the impl of even_even.
  REP(i, 0, 2) REP(j, 0, 2) assert(ret[n - 2 - i][m - 2 - j] == '.');
  if (edge_a < 2 || edge_b < 2) {
    // No point of creating 3x3. Abort.
    return vector<string>();
  }
  REP(i, 0, edge_b - 2) {
    ret[2 * i][m / 2 * 2] = '^';
    ret[2 * i + 1][m / 2 * 2] = 'v';
  }
  REP(i, 0, edge_a - 2) {
    ret[n / 2 * 2][2 * i] = '<';
    ret[n / 2 * 2][2 * i + 1] = '>';
  }
  REP(i, 0, 3) {
    REP(j, 0, 3) {
      char c = "<>^^.vv<>"[3 * i + j];
      ret[n - 3 + i][m - 3 + j] = c;
    }
  }
  return ret;
}
vector<string> odd_odd(int n, int m, int a, int b) {
  assert (n % 2 == 1 && m % 2 == 1);
  vector<string> ret(n, string(m, '.'));
  int edge_b = min(b, n / 2);
  int vero_b = b - edge_b;
  int edge_a = min(a, m / 2);
  int vero_a = a - edge_a;
  if (not is_ok_even_even(n - 1, m - 1, vero_a, vero_b)) {
    if (n > 1 && m > 1) {
      return with3x3(n, m, a, b);
    }
    return vector<string>();
  }
  vector<string> vero = even_even(n - 1, m - 1, vero_a, vero_b);
  REP(i, 0, n - 1) {
    REP(j, 0, m - 1) {
      ret[i][j] = vero[i][j];
    }
  }
  REP(i, 0, edge_b) {
    ret[2 * i][m - 1] = '^';
    ret[2 * i + 1][m - 1] = 'v';
  }
  REP(i, 0, edge_a) {
    ret[n - 1][2 * i] = '<';
    ret[n - 1][2 * i + 1] = '>';
  }
  return ret;
}
vector<string> solve(int n, int m, int a, int b) {
  if (n * m < 2 * a + 2 * b) {
    return vector<string>();
  }
  if (n % 2 == 0 && m % 2 == 0) {
    return even_even(n, m, a, b);
  }
  if (n % 2 == 0 || m % 2 == 0) {
    if (n % 2 == 0) {
      return even_odd(n, m, a, b);
    }
    vector<string> tr = even_odd(m, n, b, a);
    return trans(tr);
  }
  return odd_odd(n, m, a, b);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m >> a >> b;
  vector<string> ans;
  ans = solve(n, m, a, b);
  if (ans.size() == 0) {
    fail();
  }
  cout << "YES\n";
  REP(i, 0, n) {
    cout << ans[i] << "\n";
  }
}
