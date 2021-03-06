/*
 * Fast fourier transformation. n must be a power of 2.
 * header requirement: vector, complex
 * Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/421507)
 */
class FFT {
private:
  static const double pi = 3.141592653589793238463;
  typedef std::complex<double> comp;
  static void
  inplace_internal_fft(
		       comp const *f,
		       comp *output,
		       comp const *ztbl,
		       int x,
		       int fstart,
		       int fstep,
		       int n,
		       int ostart) {
    if (n == 1) {
      output[ostart] = f[fstart];
      return;
    }
    inplace_internal_fft(f, output, ztbl, x + 1,
			 fstart, 2 * fstep, n / 2, ostart);
    inplace_internal_fft(f, output, ztbl, x + 1,
			 fstart + fstep, 2 * fstep, n / 2, ostart + n / 2);
    comp zeta = ztbl[x];
    comp pzeta = 1;
    for (int i = 0; i < n / 2; ++i) {
      comp f0 = output[ostart + i];
      comp f1 = output[ostart + i + n / 2];
      output[ostart + i] = f0 + pzeta * f1;
      output[ostart + i + n / 2] = f0 - pzeta * f1;
      pzeta *= zeta;
    }
    return;
  }
public:
  static int ceil_pow2(int n) {
    while (n & (n -1)) {
      n += n & (-n);
    }
    return n;
  }
  static std::vector<comp> transform(std::vector<comp> f, int n) {
    int p = __builtin_popcount(n - 1); // n = 2^p
    std::vector<comp> ztbl(p);
    for (int i = 0; i < p; ++i) {
      int d = n >> i;
      comp zeta = comp(cos(2 * pi / d), sin(2 * pi / d));
      ztbl[i] = zeta;
    }
    std::vector<comp> output(n);
    inplace_internal_fft(&f[0], &output[0], &ztbl[0], 0, 0, 1, n, 0);
    return output;
  }

  static std::vector<comp> inverse_transform(std::vector<comp> f, int n) {
    int p = __builtin_popcount(n - 1); // n = 2^p
    std::vector<comp> ztbl(p);
    for (int i = 0; i < p; ++i) {
      int d = n >> i;
      comp zeta = comp(cos(2 * pi / d), - sin(2 * pi / d));
      ztbl[i] = zeta;
    }
    std::vector<comp> output(n);
    inplace_internal_fft(&f[0], &output[0], &ztbl[0], 0, 0, 1, n, 0);
    return output;
  }
};
