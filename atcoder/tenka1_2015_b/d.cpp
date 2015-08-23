#include <algorithm>
#include <cassert>
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
typedef pair<int, int> PI;
const double EPS=1e-9;


const int N = 10010;
string p[N];

string conv(string s) {
  string ret;
  REP(i, 0, s.length()) {
    char d;
    switch(s[i]) {
    case 'O': case 'D':
      d = '0';
      break;
    case 'I':
      d = '1';
      break;
    case 'Z':
      d = '2';
      break;
    case 'E':
      d = '3';
      break;
    case 'h':
      d = '4';
      break;
    case 's':
      d = '5';
      break;
    case 'q':
      d = '6';
      break;
    case 'L':
      d = '7';
      break;
    case 'B':
      d = '8';
      break;
    case 'G':
      d = '9';
      break;
    default:
      assert(0);
    }
    ret += d;
  }
  reverse(ret.begin(), ret.end());
  return ret;
}

struct comp {
  bool operator()(const string &s, const string &t) {
    return s + t > t + s;
  }
};

const int W = 210;

string dp[W];

int main(void){
  int n, d;
  cin >> d >> n;
  REP(i, 0, n) {
    string s;
    cin >> s;
    p[i] = conv(s);
  }
  sort(p, p + n, comp());
  REP(i, 0, W) {
    dp[i] = "-";
  }
  dp[0] = "";
  REP(i, 0, n) {
    int pl = p[i].length();
    for (int j = W - 1; j >= pl; --j) {
      string ma = dp[j];
      if (dp[j - pl] != "-") {
	ma = max(ma, dp[j - pl] + p[i]);
      }
      dp[j] = ma;
    }
  }
  string nz = "-", hz = "-";
  REP(j, 1, d + 1) {
    if (dp[j] != "-") {
      if (dp[j][0] == '0') {
	hz = max(hz, dp[j]);
      } else {
	nz = dp[j];
      }
    }
  }
  string ret = nz;
  
  if (ret == "-") {
    ret = hz;
  }
  if (ret[0] == '0') {
    //Add point, remove trailing zeroes.
    ret.insert(1, ".");
    while (ret[ret.length()-1] == '0') {
      ret = ret.substr(0, ret.length() - 1);
    }
    if (ret[ret.length()-1] == '.') {
      ret = "0";
    }
  }
  cout << ret << endl;
}
