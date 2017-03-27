#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

/* http://d.hatena.ne.jp/komiyam/20120118/1326812613 */
std::vector<int> z_array(const std::string &str) {
  int len = str.length();
  std::vector<int> z(len);
  z[0] = len;
  for (int i = 1, left = 0, right = 0; i < len; ++i){
    if (i > right) {
      left = right = i;
      for(; right < len && str[right - left] == str[right]; ++right) {}
      z[i] = right - left;
      right--;
    }
    else{
      int k = i - left;
      if(z[k] < right - i + 1){
	z[i] = z[k];
      }
      else{
	left = i;
	for(;right < len && str[right - left] == str[right]; right++);
	z[i] = right - left;
	right--;
      }
    }
  }
  return z;
}

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

bool all_equal(const string &w) {
  int n = w.length();
  assert (n >= 1);
  REP(i, 1, n) {
    if (w[i] != w[0]) {
      return false;
    }
  }
  return true;
}

vector<bool> make_good(const vector<int> &za) {
  int n = za.size();
  vector<bool> ret(n + 1, true);
  REP(i, 1, n) {
    if (za[i] >= i) {
      for (int j = 2 * i; j <= za[i] + i && j <= n; j += i) {
	ret[j] = false;
      }
    }
  }
  return ret;
}

int main(void){
  string w;
  cin >> w;
  // Check if all the chars in w are equal
  if (all_equal(w)) {
    cout << w.length() << endl << 1 << endl;
    return 0;
  }
  // Check if w is a repetition
  int n = w.length();
  VI za = z_array(w);
  bool is_rep = false;
  int replen = -1;
  REP(i, 2, n) {
    if (n % i != 0) { continue; }
    // Is w a repetition of w[0 .. i]?
    if (za[i] >= n - i) {
      is_rep = true;
      replen = i;
      break;
    }
  }
  if (not is_rep) {
    cout << 1 << endl << 1 << endl;
    return 0;
  }
  // Check each position
  string rev_w = w;
  reverse(rev_w.begin(), rev_w.end());
  VI zr = z_array(rev_w);
  int cnt = 0;
  vector<bool> ga = make_good(za);
  vector<bool> gr = make_good(zr);
  
  REP(i, 1, n) {
    // w[0..i], w[i..n] are both good?
    if (ga[i] && gr[n - i]) {
      cnt += 1;
    }
  }
  cout << 2 << endl;
  cout << cnt << endl;
}
