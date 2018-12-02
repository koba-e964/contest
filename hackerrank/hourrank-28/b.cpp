#include <iostream>
#include <string>
#include <fstream>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

const int DEBUG = 0;
#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

vector<string> split_string(string);

const int N = 19;
ll dp[N][1 << N];
ll ris[1 << N];

// Complete the maximumElegance function below.
int maximumElegance(int q, vector<string> s, vector<ll> b) {
  // Return an integer denoting the maximum elegance which can be obtained by Diane.
  int n = s.size();
  vector<VI> use(n, VI(10));
  REP(i, 0, n) {
    REP(j, 0, s[i].length()) {
      use[i][s[i][j] - '0'] += 1;
    }
  }
  ll ma = 0;
  REP(i, 0, n) {
    dp[i][1 << i] = b[i];
  }
  REP(bits, 0, 1 << n) {
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) continue;
      REP(j, 0, n) {
	if (i == j) continue;
	if ((bits & 1 << j) == 0) continue;
	int prev = bits ^ 1 << j;
	dp[j][bits] = max(dp[j][bits], dp[i][prev] + (b[i] ^ b[j]));
      }
    }
  }
  REP(bits, 0, 1 << n) {
    ll ma = 0;
    REP(i, 0, n) {
      ma = max(ma, dp[i][bits]);
    }
    ris[bits] = ma;
  }
  REP(bits, 0, 1 << n) {
    VI cons(10);
    REP(i, 0, n) {
      if (bits & 1 << i) {
	REP(j, 0, 10) {
	  cons[j] += use[i][j];
	}
      }
    }
    bool ok = 1;
    REP(i, 0, 10)
      if (cons[i] > q) {
	ok = false;
	break;
      }
    if (not ok) continue;
    if (DEBUG) {
      REP(i, 0, n) {
	if (bits & 1 << i) cerr << " " << b[i];
      }
      cerr << endl;
    }
    ma = max(ma, ris[bits]);
  }
  return ma;
}

int main()
{
    ofstream fout(getenv("OUTPUT_PATH"));

    string nq_temp;
    getline(cin, nq_temp);

    vector<string> nq = split_string(nq_temp);

    int n = stoi(nq[0]);

    int q = stoi(nq[1]);

    string b_temp_temp;
    getline(cin, b_temp_temp);

    vector<string> b_temp = split_string(b_temp_temp);

    vector<ll> b(n);

    for (int i = 0; i < n; i++) {
        int b_item = stoi(b_temp[i]);

        b[i] = b_item;
    }

    vector<string> s(n);

    for (int i = 0; i < n; i++) {
        string s_item;
        getline(cin, s_item);

        s[i] = s_item;
    }

    int result = maximumElegance(q, s, b);

    fout << result << "\n";

    fout.close();

    return 0;
}

vector<string> split_string(string input_string) {
    string::iterator new_end = unique(input_string.begin(), input_string.end(), [] (const char &x, const char &y) {
        return x == y and x == ' ';
    });

    input_string.erase(new_end, input_string.end());

    while (input_string[input_string.length() - 1] == ' ') {
        input_string.pop_back();
    }

    vector<string> splits;
    char delimiter = ' ';

    size_t i = 0;
    size_t pos = input_string.find(delimiter);

    while (pos != string::npos) {
        splits.push_back(input_string.substr(i, pos - i));

        i = pos + 1;
        pos = input_string.find(delimiter, i);
    }

    splits.push_back(input_string.substr(i, min(pos, input_string.length()) - i + 1));

    return splits;
}
