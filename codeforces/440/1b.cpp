#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int n;
#ifdef LOCAL
const int N = 5124;
int p[N];
int b[N];
int num_query;
void init(void) {
  set<int> s;
  REP(i, 0, n) {
    cin >> p[i];
    assert (0 <= p[i] && p[i] < n);
    s.insert(p[i]);
  }
  assert (s.size() == n);
  REP(i, 0, n) {
    b[p[i]] = i;
  }
}
int ask(int i, int j) {
  num_query += 1;
  return p[i] ^ b[j];
}
void answer(ll ans, const VI &output) {
  cerr << "ans = " << ans << endl;
  VI inv(n);
  set<int> s;
  bool valid = true;
  REP(i, 0, n) {
    if (output[i] < 0 || output[i] >= n) {
      valid = false;
      break;
    }
    inv[output[i]] = i;
    s.insert(output[i]);
  }
  // is it a valid permutation?
  if (s.size() != n) {
    valid = false;
  }
  if (not valid) {
    cerr << "Not a valid permutation:";
    REP(i, 0, n) {
      cerr << " " << output[i];
    }
    cerr << endl;
    return;
  }
  if (num_query > 2 * n) {
    cerr << "WARNING too many queries: " << num_query << " (limit " << 2 * n
	 << ")" << endl;
  } else {
    cerr << "Query " << num_query << " times OK" << endl;
  }
  int err = 0;
  REP(i, 0, n) {
    REP(j, 0, n) {
      int expected = p[i] ^ b[j];
      int got = output[i] ^ inv[j];
      if (got != expected) {
	cerr << "(" << i << ", " << j << ") differ: expected = " << expected
	     << " got = " << got << endl;
	err += 1;
      }
    }
  }
  if (err == 0) {
    cerr << "OK" << endl;
  } else {
    cerr << err << " cell(s) differ(s)" << endl;
  }
}
#else
void init(void) {}
int ask(int i, int j) {
  cout << "? " << i << " " << j << endl;
  int t;
  cin >> t;
  return t;
}
void answer(ll ans, const VI &output) {
  cout << "!" << endl;
  cout << ans << endl;
  REP(i, 0, n) {
    cout << output[i] << (i == n - 1 ? "\n" : " ");
  }
}
#endif


int surmise(const VI &v) {
  int n = v.size();
  int p = n;
  while (true) {
    int m = p & (p - 1);
    if (m == 0) {
      break;
    }
    p = m;
  }
  VI cur = v;
  int ret = 0;
  for (int i = p; i >= 1; i /= 2) {
    VI t0, t1;
    if ((n & i) == 0) {
      if (cur.size() > 0 && (cur[0] & i)) {
	ret ^= i;
      }
      continue;
    }
    REP(j, 0, cur.size()) {
      if (cur[j] & i) {
	t1.push_back(cur[j] ^ i);
      } else {
	t0.push_back(cur[j]);
      }
    }
    if ((int) t0.size() == i) {
      cur = t1;
    } else {
      cur = t0;
      ret ^= i;
    }
  }
  return ret;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  init();
  VI row(n), col(n);
  REP(i, 0, n) {
    row[i] = ask(0, i);
  }
  col[0] = row[0];
  REP(i, 1, n) {
    col[i] = ask(i, 0);
  }
  // We can surmise p[0] by row.as_set()
  int start = surmise(row);
  VI p0_cand;
  int end = start + (n & (-n));
  cerr << "searching [" << start << ", " << end << ")" << endl;
  REP(cand, start, end) {
    VI ans(n);
    REP(i, 0, n) {
      ans[row[i] ^ cand] = i;
    }
    // Is row[0] = col[0] valid?
    bool ok = true;
    if (ans[0] != cand) {
      ok = false;
    }
    // Is ans consistent with col?
    REP(i, 0, n) {
      if (not ok) {
	break;
      }
      if (col[i] != (ans[i] ^ col[0] ^ ans[0])) {
	ok = false;
      }
    }
    if (ok) {
      p0_cand.push_back(cand);
    }
  }
  assert (p0_cand.size() >= 1);
  int p0 = p0_cand[0];

  cerr << "p0 = " << p0 << endl;
  VI ans(n);
  REP(i, 0, n) {
    ans[row[i] ^ p0] = i;
  }
  answer(p0_cand.size(), ans);
}
