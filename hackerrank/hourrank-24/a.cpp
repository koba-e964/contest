#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

bool aa(string a, string b) {
  REP(i, 0, a.size()) {
    REP(j, 0, b.size()) {
      if (a[i] == b[j]) return 1;
    }
  }
  return 0;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  int ma = max(0, 6 - n);
  int cur = 0;
  string numbers = "0123456789",
    lower_case = "abcdefghijklmnopqrstuvwxyz",
    upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    special_characters = "!@#$%^&*()-+";
  if (not aa(s, numbers)) cur++;
  if (not aa(s, lower_case)) cur++;
  if (not aa(s, upper_case)) cur++;
  if (not aa(s, special_characters)) cur++;
  cout << max(ma, cur) << endl;
}
