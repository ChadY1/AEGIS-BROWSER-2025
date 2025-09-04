import 'package:flutter/material.dart';
import 'package:flutter_inappwebview/flutter_inappwebview.dart';
import 'search_bars.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const AegisApp());
}

class AegisApp extends StatelessWidget {
  const AegisApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        appBar: AppBar(title: const Text('Aegis')),
        body: const Column(children: [
          Expanded(flex: 1, child: SearchBars()),
          Expanded(flex: 5, child: BrowserView()),
        ]),
      ),
    );
  }
}

class BrowserView extends StatefulWidget { const BrowserView({super.key}); @override State<BrowserView> createState()=>_BrowserViewState(); }
class _BrowserViewState extends State<BrowserView> {
  InAppWebViewController? ctrl;
  @override
  Widget build(BuildContext context) {
    return InAppWebView(
      initialSettings: InAppWebViewSettings(
        thirdPartyCookiesEnabled: false,
        transparentBackground: true,
        javaScriptEnabled: true,
        useShouldInterceptRequest: true,
      ),
      onWebViewCreated: (c){ ctrl=c; },
      shouldInterceptRequest: (c, r) async {
        if (r.url.toString().contains('doubleclick')) {
          return WebResourceResponse(contentType: 'text/plain', data: const Stream.empty());
        }
        return null;
      },
      onLoadStop: (c, u) async {
        // TODO: save to local encrypted store
      },
    );
  }
}
