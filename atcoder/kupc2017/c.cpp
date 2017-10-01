#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const int N = 1123;
int a, n;
int c[N];

bool ok(int bias, int idx, int len) {
  // Positive?
  {
    int cur = 0;
    bool pos = true;
    for (int i = n - 1; i >= idx; --i) {
      cur = a * cur + c[i];
      if (i < len) {
	cur -= 1;
      }
      if (cur >= 100) {
	break;
      }
      if (cur < -10) {
	pos = false;
	break;
      }
    }
    pos &= cur - bias >= 0;
    if (not pos) {
      return false;
    }
  }
  // Not exceeding?
  {
    int cur = 0;
    bool inrange = true;
    for (int i = n - 1; i >= idx; --i) {
      cur = a * cur + c[i];
      if (i < len) {
	cur -= 26;
      }
      if (cur >= 100) {
	inrange = false;
	break;
      }
      if (cur < -10) {
	break;
      }
    }
    inrange &= cur - bias <= 0;
    if (not inrange) {
      return false;
    }
  }
  return true;
}

bool is_valid_length(int mid) {
  return ok(0, 0, mid);
}

int main(void) {
  string s;
  cin >> a >> s;
  n = s.length();
  REP(i, 0, n) {
    c[i] = s[i] - 'a' + 1;
  }
  // Calculate the length
  int lo = 0;
  int hi = n;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    if (is_valid_length(mid)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  int cur = 0;
  VI ans(hi);
  REP(i, 0, hi) {
    bool found = false;
    for (int j = 26; j >= 1; --j) {
      int diff = j + cur - c[i];
      if (diff % a != 0) { continue; }
      if (ok(diff / a, i + 1, hi)) {
	ans[i] = j;
	cur = diff / a;
	found = true;
	break;
      }
    }
    assert (found);
  }
  string ret;
  REP(i, 0, hi) {
    ret += (char)(ans[i] + 'a' - 1);
  }
  cout << ret << "\n";
}
